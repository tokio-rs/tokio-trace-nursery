use opentelemetry::api::IdGenerator;
use opentelemetry::{api, sdk};
use std::any::TypeId;
use std::fmt;
use std::marker;
use std::time::SystemTime;
use tracing_core::span::{self, Attributes, Id, Record};
use tracing_core::{field, Event, Subscriber};
#[cfg(feature = "tracing-log")]
use tracing_log::NormalizeEvent;
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::Layer;

static SPAN_NAME_FIELD: &str = "otel.name";

/// An [OpenTelemetry] propagation layer for use in a project that uses
/// [tracing].
///
/// [OpenTelemetry]: https://opentelemetry.io
/// [tracing]: https://github.com/tokio-rs/tracing
pub struct OpenTelemetryLayer<S, T: api::Tracer> {
    tracer: T,
    sampler: Box<dyn api::Sampler>,
    id_generator: sdk::IdGenerator,

    get_context: WithContext,
    _registry: marker::PhantomData<S>,
}

impl<S> Default for OpenTelemetryLayer<S, api::NoopTracer>
where
    S: Subscriber + for<'span> LookupSpan<'span>,
{
    fn default() -> Self {
        OpenTelemetryLayer::new(api::NoopTracer {}, sdk::Sampler::AlwaysOn)
    }
}

/// Construct a layer to track spans via [OpenTelemetry].
///
/// [OpenTelemetry]: https://opentelemetry.io
///
/// # Examples
///
/// ```rust,no_run
/// use tracing_subscriber::layer::SubscriberExt;
/// use tracing_subscriber::Registry;
///
/// // Use the tracing subscriber `Registry`, or any other subscriber
/// // that impls `LookupSpan`
/// let subscriber = Registry::default()
///     .with(tracing_opentelemetry::layer());
/// # drop(subscriber);
/// ```
pub fn layer<S>() -> OpenTelemetryLayer<S, api::NoopTracer>
where
    S: Subscriber + for<'span> LookupSpan<'span>,
{
    OpenTelemetryLayer::default()
}

// this function "remembers" the types of the subscriber so that we
// can downcast to something aware of them without knowing those
// types at the callsite.
//
// See https://github.com/tokio-rs/tracing/blob/4dad420ee1d4607bad79270c1520673fa6266a3d/tracing-error/src/layer.rs
pub(crate) struct WithContext(
    fn(&tracing::Dispatch, &span::Id, f: &mut dyn FnMut(&mut api::SpanBuilder, &dyn api::Sampler)),
);

impl WithContext {
    // This function allows a function to be called in the context of the
    // "remembered" subscriber.
    pub(crate) fn with_context<'a>(
        &self,
        dispatch: &'a tracing::Dispatch,
        id: &span::Id,
        mut f: impl FnMut(&mut api::SpanBuilder, &dyn api::Sampler),
    ) {
        (self.0)(dispatch, id, &mut f)
    }
}

pub(crate) fn build_span_context(
    builder: &mut api::SpanBuilder,
    sampler: &dyn api::Sampler,
) -> api::SpanContext {
    let span_id = builder.span_id.expect("Builders must have id");
    let (trace_id, trace_flags) = builder
        .parent_context
        .as_ref()
        .filter(|parent_context| parent_context.is_valid())
        .map(|parent_context| (parent_context.trace_id(), parent_context.trace_flags()))
        .unwrap_or_else(|| {
            let trace_id = builder.trace_id.expect("trace_id should exist");

            // ensure sampling decision is recorded so all span contexts have consistent flags
            let sampling_decision = if let Some(result) = builder.sampling_result.as_ref() {
                result.decision.clone()
            } else {
                let mut result = sampler.should_sample(
                    builder.parent_context.as_ref(),
                    trace_id,
                    &builder.name,
                    builder
                        .span_kind
                        .as_ref()
                        .unwrap_or(&api::SpanKind::Internal),
                    builder.attributes.as_ref().unwrap_or(&Vec::new()),
                    builder.links.as_ref().unwrap_or(&Vec::new()),
                );

                // Record additional attributes resulting from sampling
                if let Some(attributes) = &mut builder.attributes {
                    attributes.append(&mut result.attributes)
                } else {
                    builder.attributes = Some(result.attributes);
                }

                result.decision
            };

            let trace_flags = if sampling_decision == api::SamplingDecision::RecordAndSampled {
                api::TRACE_FLAG_SAMPLED
            } else {
                0
            };

            (trace_id, trace_flags)
        });

    api::SpanContext::new(trace_id, span_id, trace_flags, false)
}

struct SpanEventVisitor<'a>(&'a mut api::Event);

impl<'a> field::Visit for SpanEventVisitor<'a> {
    /// Record events on the underlying OpenTelemetry [`Span`] from `&str` values.
    ///
    /// [`Span`]: https://docs.rs/opentelemetry/latest/opentelemetry/api/trace/span/trait.Span.html
    fn record_str(&mut self, field: &field::Field, value: &str) {
        match field.name() {
            "message" => self.0.name = value.to_string(),
            // Skip fields that are actually log metadata that have already been handled
            #[cfg(feature = "tracing-log")]
            name if name.starts_with("log.") => (),
            name => {
                self.0.attributes.push(api::KeyValue::new(name, value));
            }
        }
    }

    /// Record events on the underlying OpenTelemetry [`Span`] from values that
    /// implement Debug.
    ///
    /// [`Span`]: https://docs.rs/opentelemetry/latest/opentelemetry/api/trace/span/trait.Span.html
    fn record_debug(&mut self, field: &field::Field, value: &dyn fmt::Debug) {
        match field.name() {
            "message" => self.0.name = format!("{:?}", value),
            // Skip fields that are actually log metadata that have already been handled
            #[cfg(feature = "tracing-log")]
            name if name.starts_with("log.") => (),
            name => {
                self.0
                    .attributes
                    .push(api::KeyValue::new(name, format!("{:?}", value)));
            }
        }
    }
}

struct SpanAttributeVisitor<'a>(&'a mut api::SpanBuilder);

impl<'a> field::Visit for SpanAttributeVisitor<'a> {
    /// Set attributes on the underlying OpenTelemetry [`Span`] from `&str` values.
    ///
    /// [`Span`]: https://docs.rs/opentelemetry/latest/opentelemetry/api/trace/span/trait.Span.html
    fn record_str(&mut self, field: &field::Field, value: &str) {
        if field.name() == SPAN_NAME_FIELD {
            self.0.name = value.to_string();
        } else {
            let attribute = api::KeyValue::new(field.name(), value);
            if let Some(attributes) = &mut self.0.attributes {
                attributes.push(attribute);
            } else {
                self.0.attributes = Some(vec![attribute]);
            }
        }
    }

    /// Set attributes on the underlying OpenTelemetry [`Span`] from values that
    /// implement Debug.
    ///
    /// [`Span`]: https://docs.rs/opentelemetry/latest/opentelemetry/api/trace/span/trait.Span.html
    fn record_debug(&mut self, field: &field::Field, value: &dyn fmt::Debug) {
        if field.name() == SPAN_NAME_FIELD {
            self.0.name = format!("{:?}", value);
        } else {
            let attribute = api::Key::new(field.name()).string(format!("{:?}", value));
            if let Some(attributes) = &mut self.0.attributes {
                attributes.push(attribute);
            } else {
                self.0.attributes = Some(vec![attribute]);
            }
        }
    }
}

impl<S, T> OpenTelemetryLayer<S, T>
where
    S: Subscriber + for<'span> LookupSpan<'span>,
    T: api::Tracer + 'static,
{
    /// Set the [`Tracer`] and [`Sampler`] that this layer will use to produce and
    /// track OpenTelemetry [`Span`]s.
    ///
    /// [`Tracer`]: https://docs.rs/opentelemetry/latest/opentelemetry/api/trace/tracer/trait.Tracer.html
    /// [`Sampler`]: https://docs.rs/opentelemetry/latest/opentelemetry/api/trace/sampler/trait.Sampler.html
    /// [`Span`]: https://docs.rs/opentelemetry/latest/opentelemetry/api/trace/span/trait.Span.html
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use opentelemetry::{api::Provider, sdk};
    /// use tracing_opentelemetry::OpenTelemetryLayer;
    /// use tracing_subscriber::layer::SubscriberExt;
    /// use tracing_subscriber::Registry;
    ///
    /// // Create a jaeger exporter for a `trace-demo` service.
    /// let exporter = opentelemetry_jaeger::Exporter::builder()
    ///     .with_agent_endpoint("127.0.0.1:6831".parse().unwrap())
    ///     .with_process(opentelemetry_jaeger::Process {
    ///         service_name: "trace_demo".to_string(),
    ///         tags: Vec::new(),
    ///     })
    ///     .init().expect("Error initializing Jaeger exporter");
    ///
    /// // Build a provider from the jaeger exporter that always samples.
    /// let provider = sdk::Provider::builder()
    ///     .with_simple_exporter(exporter)
    ///     .with_config(sdk::Config {
    ///         default_sampler: Box::new(sdk::Sampler::AlwaysOn),
    ///         ..Default::default()
    ///     })
    ///     .build();
    ///
    /// // Get a tracer from the provider for a component
    /// let tracer = provider.get_tracer("component-name");
    ///
    /// // The probability sampler can be used to export a percentage of spans
    /// let sampler = sdk::Sampler::Probability(0.33);
    ///
    /// // Create a layer with the configured tracer
    /// let otel_layer = OpenTelemetryLayer::new(tracer, sampler);
    ///
    /// // Use the tracing subscriber `Registry`, or any other subscriber
    /// // that impls `LookupSpan`
    /// let subscriber = Registry::default()
    ///     .with(otel_layer);
    /// # drop(subscriber);
    /// ```
    pub fn new<Sampler>(tracer: T, sampler: Sampler) -> Self
    where
        Sampler: api::Sampler + 'static,
    {
        OpenTelemetryLayer {
            tracer,
            sampler: Box::new(sampler),
            id_generator: sdk::IdGenerator::default(),
            get_context: WithContext(Self::get_context),
            _registry: marker::PhantomData,
        }
    }

    /// Set the [`Tracer`] that this layer will use to produce and track
    /// OpenTelemetry [`Span`]s.
    ///
    /// [`Tracer`]: https://docs.rs/opentelemetry/latest/opentelemetry/api/trace/tracer/trait.Tracer.html
    /// [`Span`]: https://docs.rs/opentelemetry/latest/opentelemetry/api/trace/span/trait.Span.html
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use opentelemetry::{api::Provider, sdk};
    /// use tracing_subscriber::layer::SubscriberExt;
    /// use tracing_subscriber::Registry;
    ///
    /// // Create a jaeger exporter for a `trace-demo` service.
    /// let exporter = opentelemetry_jaeger::Exporter::builder()
    ///     .with_agent_endpoint("127.0.0.1:6831".parse().unwrap())
    ///     .with_process(opentelemetry_jaeger::Process {
    ///         service_name: "trace_demo".to_string(),
    ///         tags: Vec::new(),
    ///     })
    ///     .init().expect("Error initializing Jaeger exporter");
    ///
    /// // Build a provider from the jaeger exporter that always samples.
    /// let provider = sdk::Provider::builder()
    ///     .with_simple_exporter(exporter)
    ///     .with_config(sdk::Config {
    ///         default_sampler: Box::new(sdk::Sampler::AlwaysOn),
    ///         ..Default::default()
    ///     })
    ///     .build();
    ///
    /// // Get a tracer from the provider for a component
    /// let tracer = provider.get_tracer("component-name");
    ///
    /// // Create a layer with the configured tracer
    /// let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);
    ///
    /// // Use the tracing subscriber `Registry`, or any other subscriber
    /// // that impls `LookupSpan`
    /// let subscriber = Registry::default()
    ///     .with(otel_layer);
    /// # drop(subscriber);
    /// ```
    pub fn with_tracer<Tracer>(self, tracer: Tracer) -> OpenTelemetryLayer<S, Tracer>
    where
        Tracer: api::Tracer + 'static,
    {
        OpenTelemetryLayer {
            tracer,
            sampler: self.sampler,
            id_generator: self.id_generator,
            get_context: WithContext(OpenTelemetryLayer::<S, Tracer>::get_context),
            _registry: self._registry,
        }
    }

    /// Set the [`Sampler`] to configure the logic around which [`Span`]s are
    /// exported.
    ///
    /// [`Sampler`]: https://docs.rs/opentelemetry/latest/opentelemetry/api/trace/sampler/trait.Sampler.html
    /// [`Span`]: https://docs.rs/opentelemetry/latest/opentelemetry/api/trace/span/trait.Span.html
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use opentelemetry::sdk;
    /// use tracing_subscriber::layer::SubscriberExt;
    /// use tracing_subscriber::Registry;
    ///
    /// // The probability sampler can be used to export a percentage of spans
    /// let sampler = sdk::Sampler::Probability(0.33);
    ///
    /// // Create a layer with the configured sampler
    /// let otel_layer = tracing_opentelemetry::layer().with_sampler(sampler);
    ///
    /// // Use the tracing subscriber `Registry`, or any other subscriber
    /// // that impls `LookupSpan`
    /// let subscriber = Registry::default()
    ///     .with(otel_layer);
    /// # drop(subscriber);
    /// ```
    pub fn with_sampler<Sampler>(self, sampler: Sampler) -> Self
    where
        Sampler: api::Sampler + 'static,
    {
        OpenTelemetryLayer {
            sampler: Box::new(sampler),
            ..self
        }
    }

    /// Retrieve the parent OpenTelemetry [`SpanContext`] from the current
    /// tracing [`span`] through the [`Registry`]. This [`SpanContext`]
    /// links spans to their parent for proper hierarchical visualization.
    ///
    /// [`SpanContext`]: https://docs.rs/opentelemetry/latest/opentelemetry/api/trace/span_context/struct.SpanContext.html
    /// [`span`]: https://docs.rs/tracing/latest/tracing/struct.Span.html
    /// [`Registry`]: https://docs.rs/tracing-subscriber/latest/tracing_subscriber/struct.Registry.html
    fn parent_span_context(
        &self,
        attrs: &Attributes<'_>,
        ctx: &Context<'_, S>,
    ) -> Option<api::SpanContext> {
        // If a span is specified, it _should_ exist in the underlying `Registry`.
        if let Some(parent) = attrs.parent() {
            let span = ctx.span(parent).expect("Span not found, this is a bug");
            let mut extensions = span.extensions_mut();
            extensions
                .get_mut::<api::SpanBuilder>()
                .map(|builder| build_span_context(builder, self.sampler.as_ref()))
        // Else if the span is inferred from context, look up any available current span.
        } else if attrs.is_contextual() {
            ctx.lookup_current().and_then(|span| {
                let mut extensions = span.extensions_mut();
                extensions
                    .get_mut::<api::SpanBuilder>()
                    .map(|builder| build_span_context(builder, self.sampler.as_ref()))
            })
        // Explicit root spans should have no parent context.
        } else {
            None
        }
    }

    fn get_context(
        dispatch: &tracing::Dispatch,
        id: &span::Id,
        f: &mut dyn FnMut(&mut api::SpanBuilder, &dyn api::Sampler),
    ) {
        let subscriber = dispatch
            .downcast_ref::<S>()
            .expect("subscriber should downcast to expected type; this is a bug!");
        let span = subscriber
            .span(id)
            .expect("registry should have a span for the current ID");
        let layer = dispatch
            .downcast_ref::<OpenTelemetryLayer<S, T>>()
            .expect("layer should downcast to expected type; this is a bug!");

        let mut extensions = span.extensions_mut();
        if let Some(builder) = extensions.get_mut::<api::SpanBuilder>() {
            f(builder, layer.sampler.as_ref());
        }
    }
}

impl<S, T> Layer<S> for OpenTelemetryLayer<S, T>
where
    S: Subscriber + for<'span> LookupSpan<'span>,
    T: api::Tracer + 'static,
{
    /// Creates an [OpenTelemetry `Span`] for the corresponding [tracing `Span`].
    ///
    /// [OpenTelemetry `Span`]: https://docs.rs/opentelemetry/latest/opentelemetry/api/trace/span/trait.Span.html
    /// [tracing `Span`]: https://docs.rs/tracing/latest/tracing/struct.Span.html
    fn new_span(&self, attrs: &Attributes<'_>, id: &span::Id, ctx: Context<'_, S>) {
        let span = ctx.span(id).expect("Span not found, this is a bug");
        let mut extensions = span.extensions_mut();

        let mut builder = self
            .tracer
            .span_builder(attrs.metadata().name())
            .with_start_time(SystemTime::now())
            // Eagerly assign span id so children have stable parent id
            .with_span_id(self.id_generator.new_span_id());

        // Set optional parent span context from attrs
        builder.parent_context = self.parent_span_context(attrs, &ctx);

        // Ensure trace id exists so children are matched properly.
        if builder.parent_context.is_none() {
            builder.trace_id = Some(self.id_generator.new_trace_id());
        }

        attrs.record(&mut SpanAttributeVisitor(&mut builder));
        extensions.insert(builder);
    }

    /// Record OpenTelemetry [`attributes`] for the given values.
    ///
    /// [`attributes`]: https://docs.rs/opentelemetry/latest/opentelemetry/api/trace/tracer/struct.SpanBuilder.html#structfield.attributes
    fn on_record(&self, id: &Id, values: &Record<'_>, ctx: Context<'_, S>) {
        let span = ctx.span(id).expect("Span not found, this is a bug");
        let mut extensions = span.extensions_mut();
        if let Some(builder) = extensions.get_mut::<api::SpanBuilder>() {
            values.record(&mut SpanAttributeVisitor(builder));
        }
    }

    fn on_follows_from(&self, id: &Id, follows: &Id, ctx: Context<S>) {
        let span = ctx.span(id).expect("Span not found, this is a bug");
        let mut extensions = span.extensions_mut();
        let builder = extensions
            .get_mut::<api::SpanBuilder>()
            .expect("Missing SpanBuilder span extensions");

        let follows_span = ctx
            .span(follows)
            .expect("Span to follow not found, this is a bug");
        let mut follows_extensions = follows_span.extensions_mut();
        let follows_builder = follows_extensions
            .get_mut::<api::SpanBuilder>()
            .expect("Missing SpanBuilder span extensions");

        let follows_context = build_span_context(follows_builder, self.sampler.as_ref());
        let follows_link = api::Link::new(follows_context, Vec::new());
        if let Some(ref mut links) = builder.links {
            links.push(follows_link);
        } else {
            builder.links = Some(vec![follows_link]);
        }
    }

    /// Records OpenTelemetry [`Event`] data on event.
    ///
    /// Note: an [`ERROR`]-level event will also set the OpenTelemetry span status code to
    /// [`Unknown`], signaling that an error has occurred.
    ///
    /// [`Event`]: https://docs.rs/opentelemetry/latest/opentelemetry/api/trace/event/struct.Event.html
    /// [`ERROR`]: https://docs.rs/tracing/latest/tracing/struct.Level.html#associatedconstant.ERROR
    /// [`Unknown`]: https://docs.rs/opentelemetry/latest/opentelemetry/api/trace/span/enum.StatusCode.html#variant.Unknown
    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        // Ignore events that are not in the context of a span
        if let Some(span) = ctx.lookup_current() {
            // Performing read operations before getting a write lock to avoid a deadlock
            // See https://github.com/tokio-rs/tracing/issues/763
            #[cfg(feature = "tracing-log")]
            let normalized_meta = event.normalized_metadata();
            #[cfg(feature = "tracing-log")]
            let meta = normalized_meta.as_ref().unwrap_or_else(|| event.metadata());
            #[cfg(not(feature = "tracing-log"))]
            let meta = event.metadata();
            let mut otel_event = api::Event::new(
                String::new(),
                SystemTime::now(),
                vec![
                    api::Key::new("level").string(meta.level().to_string()),
                    api::Key::new("target").string(meta.target()),
                ],
            );
            event.record(&mut SpanEventVisitor(&mut otel_event));

            let mut extensions = span.extensions_mut();
            if let Some(builder) = extensions.get_mut::<api::SpanBuilder>() {
                if builder.status_code.is_none() && *meta.level() == tracing_core::Level::ERROR {
                    builder.status_code = Some(api::StatusCode::Unknown);
                }

                if let Some(ref mut events) = builder.message_events {
                    events.push(otel_event);
                } else {
                    builder.message_events = Some(vec![otel_event]);
                }
            }
        };
    }

    /// Exports an OpenTelemetry [`Span`] on close.
    ///
    /// [`Span`]: https://docs.rs/opentelemetry/latest/opentelemetry/api/trace/span/trait.Span.html
    fn on_close(&self, id: span::Id, ctx: Context<'_, S>) {
        let span = ctx.span(&id).expect("Span not found, this is a bug");
        let mut extensions = span.extensions_mut();
        if let Some(builder) = extensions.remove::<api::SpanBuilder>() {
            // Assign end time, build and start span, drop span to export
            builder.with_end_time(SystemTime::now()).start(&self.tracer);
        }
    }

    // SAFETY: this is safe because the `WithContext` function pointer is valid
    // for the lifetime of `&self`.
    unsafe fn downcast_raw(&self, id: TypeId) -> Option<*const ()> {
        match id {
            id if id == TypeId::of::<Self>() => Some(self as *const _ as *const ()),
            id if id == TypeId::of::<WithContext>() => {
                Some(&self.get_context as *const _ as *const ())
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use opentelemetry::api;
    use std::sync::{Arc, Mutex};
    use tracing_subscriber::prelude::*;

    #[derive(Debug, Clone)]
    struct TestTracer(Arc<Mutex<Option<api::SpanBuilder>>>);
    impl api::Tracer for TestTracer {
        type Span = api::NoopSpan;
        fn invalid(&self) -> Self::Span {
            api::NoopSpan::new()
        }
        fn start_from_context(&self, _name: &str, _context: &api::Context) -> Self::Span {
            self.invalid()
        }
        fn span_builder(&self, name: &str) -> api::SpanBuilder {
            api::SpanBuilder::from_name(name.to_string())
        }
        fn build_with_context(&self, builder: api::SpanBuilder, _cx: &api::Context) -> Self::Span {
            *self.0.lock().unwrap() = Some(builder);
            self.invalid()
        }
    }

    #[test]
    fn dynamic_span_names() {
        let dynamic_name = "GET http://example.com".to_string();
        let tracer = TestTracer(Arc::new(Mutex::new(None)));
        let subscriber = tracing_subscriber::registry().with(layer().with_tracer(tracer.clone()));

        tracing::subscriber::with_default(subscriber, || {
            tracing::debug_span!("static_name", otel.name = dynamic_name.as_str());
        });

        let recorded_name = tracer.0.lock().unwrap().as_ref().map(|b| b.name.clone());
        assert_eq!(recorded_name, Some(dynamic_name))
    }
}
