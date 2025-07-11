// This file is @generated by prost-build.
/// The tracing configuration specifies settings for an HTTP tracer provider used by Envoy.
///
/// Envoy may support other tracers in the future, but right now the HTTP tracer is the only one
/// supported.
///
/// .. attention::
///
///    Use of this message type has been deprecated in favor of direct use of
///    :ref:`Tracing.Http <envoy_v3_api_msg_config.trace.v3.Tracing.Http>`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tracing {
    /// Provides configuration for the HTTP tracer.
    #[prost(message, optional, tag = "1")]
    pub http: ::core::option::Option<tracing::Http>,
}
/// Nested message and enum types in `Tracing`.
pub mod tracing {
    /// Configuration for an HTTP tracer provider used by Envoy.
    ///
    /// The configuration is defined by the
    /// :ref:`HttpConnectionManager.Tracing <envoy_v3_api_msg_extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.Tracing>`
    /// :ref:`provider <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.Tracing.provider>`
    /// field.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Http {
        /// The name of the HTTP trace driver to instantiate. The name must match a
        /// supported HTTP trace driver.
        /// See the :ref:`extensions listed in typed_config below <extension_category_envoy.tracers>` for the default list of the HTTP trace driver.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// Trace driver specific configuration which must be set according to the driver being instantiated.
        /// \[#extension-category: envoy.tracers\]
        #[prost(oneof = "http::ConfigType", tags = "3")]
        pub config_type: ::core::option::Option<http::ConfigType>,
    }
    /// Nested message and enum types in `Http`.
    pub mod http {
        /// Trace driver specific configuration which must be set according to the driver being instantiated.
        /// \[#extension-category: envoy.tracers\]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ConfigType {
            #[prost(message, tag = "3")]
            TypedConfig(super::super::super::super::super::super::google::protobuf::Any),
        }
    }
    impl ::prost::Name for Http {
        const NAME: &'static str = "Http";
        const PACKAGE: &'static str = "envoy.config.trace.v3";
        fn full_name() -> ::prost::alloc::string::String {
            "envoy.config.trace.v3.Tracing.Http".into()
        }
        fn type_url() -> ::prost::alloc::string::String {
            "type.googleapis.com/envoy.config.trace.v3.Tracing.Http".into()
        }
    }
}
impl ::prost::Name for Tracing {
    const NAME: &'static str = "Tracing";
    const PACKAGE: &'static str = "envoy.config.trace.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.config.trace.v3.Tracing".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.config.trace.v3.Tracing".into()
    }
}
/// Configuration for the Remote Configuration feature.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct DatadogRemoteConfig {
    /// Frequency at which new configuration updates are queried.
    /// If no value is provided, the default value is delegated to the Datadog tracing library.
    #[prost(message, optional, tag = "1")]
    pub polling_interval: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
}
impl ::prost::Name for DatadogRemoteConfig {
    const NAME: &'static str = "DatadogRemoteConfig";
    const PACKAGE: &'static str = "envoy.config.trace.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.config.trace.v3.DatadogRemoteConfig".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.config.trace.v3.DatadogRemoteConfig".into()
    }
}
/// Configuration for the Datadog tracer.
/// \[#extension: envoy.tracers.datadog\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatadogConfig {
    /// The cluster to use for submitting traces to the Datadog agent.
    #[prost(string, tag = "1")]
    pub collector_cluster: ::prost::alloc::string::String,
    /// The name used for the service when traces are generated by envoy.
    #[prost(string, tag = "2")]
    pub service_name: ::prost::alloc::string::String,
    /// Optional hostname to use when sending spans to the collector_cluster. Useful for collectors
    /// that require a specific hostname. Defaults to :ref:`collector_cluster <envoy_v3_api_field_config.trace.v3.DatadogConfig.collector_cluster>` above.
    #[prost(string, tag = "3")]
    pub collector_hostname: ::prost::alloc::string::String,
    /// Enables and configures remote configuration.
    /// Remote Configuration allows to configure the tracer from Datadog's user interface.
    /// This feature can drastically increase the number of connections to the Datadog Agent.
    /// Each tracer regularly polls for configuration updates, and the number of tracers is the product
    /// of the number of listeners and worker threads.
    #[prost(message, optional, tag = "4")]
    pub remote_config: ::core::option::Option<DatadogRemoteConfig>,
}
impl ::prost::Name for DatadogConfig {
    const NAME: &'static str = "DatadogConfig";
    const PACKAGE: &'static str = "envoy.config.trace.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.config.trace.v3.DatadogConfig".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.config.trace.v3.DatadogConfig".into()
    }
}
/// DynamicOtConfig was used to dynamically load a tracer from a shared library
/// that implements the `OpenTracing dynamic loading API
/// <<https://github.com/opentracing/opentracing-cpp>`_.>
/// \[#not-implemented-hide:\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicOtConfig {
    /// Dynamic library implementing the `OpenTracing API
    /// <<https://github.com/opentracing/opentracing-cpp>`_.>
    #[deprecated]
    #[prost(string, tag = "1")]
    pub library: ::prost::alloc::string::String,
    /// The configuration to use when creating a tracer from the given dynamic
    /// library.
    #[deprecated]
    #[prost(message, optional, tag = "2")]
    pub config: ::core::option::Option<
        super::super::super::super::google::protobuf::Struct,
    >,
}
impl ::prost::Name for DynamicOtConfig {
    const NAME: &'static str = "DynamicOtConfig";
    const PACKAGE: &'static str = "envoy.config.trace.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.config.trace.v3.DynamicOtConfig".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.config.trace.v3.DynamicOtConfig".into()
    }
}
/// Configuration for the LightStep tracer.
/// \[#extension: envoy.tracers.lightstep\]
/// \[#not-implemented-hide:\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LightstepConfig {
    /// The cluster manager cluster that hosts the LightStep collectors.
    #[prost(string, tag = "1")]
    pub collector_cluster: ::prost::alloc::string::String,
    /// File containing the access token to the `LightStep
    /// <<https://lightstep.com/>`_> API.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub access_token_file: ::prost::alloc::string::String,
    /// Access token to the `LightStep <<https://lightstep.com/>`_> API.
    #[prost(message, optional, tag = "4")]
    pub access_token: ::core::option::Option<super::super::core::v3::DataSource>,
    /// Propagation modes to use by LightStep's tracer.
    #[prost(
        enumeration = "lightstep_config::PropagationMode",
        repeated,
        packed = "false",
        tag = "3"
    )]
    pub propagation_modes: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `LightstepConfig`.
pub mod lightstep_config {
    /// Available propagation modes
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PropagationMode {
        /// Propagate trace context in the single header x-ot-span-context.
        Envoy = 0,
        /// Propagate trace context using LightStep's native format.
        Lightstep = 1,
        /// Propagate trace context using the b3 format.
        B3 = 2,
        /// Propagation trace context using the w3 trace-context standard.
        TraceContext = 3,
    }
    impl PropagationMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Envoy => "ENVOY",
                Self::Lightstep => "LIGHTSTEP",
                Self::B3 => "B3",
                Self::TraceContext => "TRACE_CONTEXT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ENVOY" => Some(Self::Envoy),
                "LIGHTSTEP" => Some(Self::Lightstep),
                "B3" => Some(Self::B3),
                "TRACE_CONTEXT" => Some(Self::TraceContext),
                _ => None,
            }
        }
    }
}
impl ::prost::Name for LightstepConfig {
    const NAME: &'static str = "LightstepConfig";
    const PACKAGE: &'static str = "envoy.config.trace.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.config.trace.v3.LightstepConfig".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.config.trace.v3.LightstepConfig".into()
    }
}
/// Configuration for the OpenTelemetry tracer.
///   \[#extension: envoy.tracers.opentelemetry\]
/// \[#next-free-field: 7\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenTelemetryConfig {
    /// The upstream gRPC cluster that will receive OTLP traces.
    /// Note that the tracer drops traces if the server does not read data fast enough.
    /// This field can be left empty to disable reporting traces to the gRPC service.
    /// Only one of ``grpc_service``, ``http_service`` may be used.
    #[prost(message, optional, tag = "1")]
    pub grpc_service: ::core::option::Option<super::super::core::v3::GrpcService>,
    /// The upstream HTTP cluster that will receive OTLP traces.
    /// This field can be left empty to disable reporting traces to the HTTP service.
    /// Only one of ``grpc_service``, ``http_service`` may be used.
    ///
    /// .. note::
    ///
    ///    Note: The ``request_headers_to_add`` property in the OTLP HTTP exporter service
    ///    does not support the :ref:`format specifier <config_access_log_format>` as used for
    ///    :ref:`HTTP access logging <config_access_log>`.
    ///    The values configured are added as HTTP headers on the OTLP export request
    ///    without any formatting applied.
    #[prost(message, optional, tag = "3")]
    pub http_service: ::core::option::Option<super::super::core::v3::HttpService>,
    /// The name for the service. This will be populated in the ResourceSpan Resource attributes.
    /// If it is not provided, it will default to "unknown_service:envoy".
    #[prost(string, tag = "2")]
    pub service_name: ::prost::alloc::string::String,
    /// An ordered list of resource detectors
    /// \[#extension-category: envoy.tracers.opentelemetry.resource_detectors\]
    #[prost(message, repeated, tag = "4")]
    pub resource_detectors: ::prost::alloc::vec::Vec<
        super::super::core::v3::TypedExtensionConfig,
    >,
    /// Specifies the sampler to be used by the OpenTelemetry tracer.
    /// The configured sampler implements the Sampler interface defined by the OpenTelemetry specification.
    /// This field can be left empty. In this case, the default Envoy sampling decision is used.
    ///
    /// See: `OpenTelemetry sampler specification <<https://opentelemetry.io/docs/specs/otel/trace/sdk/#sampler>`_>
    /// \[#extension-category: envoy.tracers.opentelemetry.samplers\]
    #[prost(message, optional, tag = "5")]
    pub sampler: ::core::option::Option<super::super::core::v3::TypedExtensionConfig>,
    /// Envoy caches the span in memory when the OpenTelemetry backend service is temporarily unavailable.
    /// This field specifies the maximum number of spans that can be cached. If not specified, the
    /// default is 1024.
    #[prost(message, optional, tag = "6")]
    pub max_cache_size: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
}
impl ::prost::Name for OpenTelemetryConfig {
    const NAME: &'static str = "OpenTelemetryConfig";
    const PACKAGE: &'static str = "envoy.config.trace.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.config.trace.v3.OpenTelemetryConfig".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.config.trace.v3.OpenTelemetryConfig".into()
    }
}
/// Configuration structure.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TraceServiceConfig {
    /// The upstream gRPC cluster that hosts the metrics service.
    #[prost(message, optional, tag = "1")]
    pub grpc_service: ::core::option::Option<super::super::core::v3::GrpcService>,
}
impl ::prost::Name for TraceServiceConfig {
    const NAME: &'static str = "TraceServiceConfig";
    const PACKAGE: &'static str = "envoy.config.trace.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.config.trace.v3.TraceServiceConfig".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.config.trace.v3.TraceServiceConfig".into()
    }
}
/// Configuration for the SkyWalking tracer. Please note that if SkyWalking tracer is used as the
/// provider of tracing, then
/// :ref:`spawn_upstream_span <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.Tracing.spawn_upstream_span>`
/// in the tracing config must be set to true to get the correct topology and tracing data. Moreover, SkyWalking
/// Tracer does not support SkyWalking extension header (``sw8-x``) temporarily.
/// \[#extension: envoy.tracers.skywalking\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SkyWalkingConfig {
    /// SkyWalking collector service.
    #[prost(message, optional, tag = "1")]
    pub grpc_service: ::core::option::Option<super::super::core::v3::GrpcService>,
    #[prost(message, optional, tag = "2")]
    pub client_config: ::core::option::Option<ClientConfig>,
}
impl ::prost::Name for SkyWalkingConfig {
    const NAME: &'static str = "SkyWalkingConfig";
    const PACKAGE: &'static str = "envoy.config.trace.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.config.trace.v3.SkyWalkingConfig".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.config.trace.v3.SkyWalkingConfig".into()
    }
}
/// Client config for SkyWalking tracer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientConfig {
    /// Service name for SkyWalking tracer. If this field is empty, then local service cluster name
    /// that configured by :ref:`Bootstrap node <envoy_v3_api_field_config.bootstrap.v3.Bootstrap.node>`
    /// message's :ref:`cluster <envoy_v3_api_field_config.core.v3.Node.cluster>` field or command line
    /// option :option:`--service-cluster` will be used. If both this field and local service cluster
    /// name are empty, ``EnvoyProxy`` is used as the service name by default.
    #[prost(string, tag = "1")]
    pub service_name: ::prost::alloc::string::String,
    /// Service instance name for SkyWalking tracer. If this field is empty, then local service node
    /// that configured by :ref:`Bootstrap node <envoy_v3_api_field_config.bootstrap.v3.Bootstrap.node>`
    /// message's :ref:`id <envoy_v3_api_field_config.core.v3.Node.id>` field or command line  option
    /// :option:`--service-node` will be used. If both this field and local service node are empty,
    /// ``EnvoyProxy`` is used as the instance name by default.
    #[prost(string, tag = "2")]
    pub instance_name: ::prost::alloc::string::String,
    /// Envoy caches the segment in memory when the SkyWalking backend service is temporarily unavailable.
    /// This field specifies the maximum number of segments that can be cached. If not specified, the
    /// default is 1024.
    #[prost(message, optional, tag = "4")]
    pub max_cache_size: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// Authentication token config for SkyWalking. SkyWalking can use token authentication to secure
    /// that monitoring application data can be trusted. In current version, Token is considered as a
    /// simple string.
    /// \[#comment:TODO(wbpcode): Get backend token through the SDS API.\]
    #[prost(oneof = "client_config::BackendTokenSpecifier", tags = "3")]
    pub backend_token_specifier: ::core::option::Option<
        client_config::BackendTokenSpecifier,
    >,
}
/// Nested message and enum types in `ClientConfig`.
pub mod client_config {
    /// Authentication token config for SkyWalking. SkyWalking can use token authentication to secure
    /// that monitoring application data can be trusted. In current version, Token is considered as a
    /// simple string.
    /// \[#comment:TODO(wbpcode): Get backend token through the SDS API.\]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum BackendTokenSpecifier {
        /// Inline authentication token string.
        #[prost(string, tag = "3")]
        BackendToken(::prost::alloc::string::String),
    }
}
impl ::prost::Name for ClientConfig {
    const NAME: &'static str = "ClientConfig";
    const PACKAGE: &'static str = "envoy.config.trace.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.config.trace.v3.ClientConfig".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.config.trace.v3.ClientConfig".into()
    }
}
/// Configuration for the Zipkin tracer.
/// \[#extension: envoy.tracers.zipkin\]
/// \[#next-free-field: 8\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZipkinConfig {
    /// The cluster manager cluster that hosts the Zipkin collectors.
    #[prost(string, tag = "1")]
    pub collector_cluster: ::prost::alloc::string::String,
    /// The API endpoint of the Zipkin service where the spans will be sent. When
    /// using a standard Zipkin installation.
    #[prost(string, tag = "2")]
    pub collector_endpoint: ::prost::alloc::string::String,
    /// Determines whether a 128bit trace id will be used when creating a new
    /// trace instance. The default value is false, which will result in a 64 bit trace id being used.
    #[prost(bool, tag = "3")]
    pub trace_id_128bit: bool,
    /// Determines whether client and server spans will share the same span context.
    /// The default value is true.
    #[prost(message, optional, tag = "4")]
    pub shared_span_context: ::core::option::Option<
        super::super::super::super::google::protobuf::BoolValue,
    >,
    /// Determines the selected collector endpoint version.
    #[prost(enumeration = "zipkin_config::CollectorEndpointVersion", tag = "5")]
    pub collector_endpoint_version: i32,
    /// Optional hostname to use when sending spans to the collector_cluster. Useful for collectors
    /// that require a specific hostname. Defaults to :ref:`collector_cluster <envoy_v3_api_field_config.trace.v3.ZipkinConfig.collector_cluster>` above.
    #[prost(string, tag = "6")]
    pub collector_hostname: ::prost::alloc::string::String,
    /// If this is set to true, then Envoy will be treated as an independent hop in trace chain. A complete span pair will be created for a single
    /// request. Server span will be created for the downstream request and client span will be created for the related upstream request.
    /// This should be set to true in the following cases:
    ///
    /// * The Envoy Proxy is used as gateway or ingress.
    /// * The Envoy Proxy is used as sidecar but inbound traffic capturing or outbound traffic capturing is disabled.
    /// * Any case that the :ref:`start_child_span of router <envoy_v3_api_field_extensions.filters.http.router.v3.Router.start_child_span>` is set to true.
    ///
    /// .. attention::
    ///
    ///    If this is set to true, then the
    ///    :ref:`start_child_span of router <envoy_v3_api_field_extensions.filters.http.router.v3.Router.start_child_span>`
    ///    SHOULD be set to true also to ensure the correctness of trace chain.
    ///
    ///    Both this field and ``start_child_span`` are deprecated by the
    ///    :ref:`spawn_upstream_span <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.Tracing.spawn_upstream_span>`.
    ///    Please use that ``spawn_upstream_span`` field to control the span creation.
    #[deprecated]
    #[prost(bool, tag = "7")]
    pub split_spans_for_request: bool,
}
/// Nested message and enum types in `ZipkinConfig`.
pub mod zipkin_config {
    /// Available Zipkin collector endpoint versions.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum CollectorEndpointVersion {
        /// Zipkin API v1, JSON over HTTP.
        /// [#comment: The default implementation of Zipkin client before this field is added was only v1
        /// and the way user configure this was by not explicitly specifying the version. Consequently,
        /// before this is added, the corresponding Zipkin collector expected to receive v1 payload.
        /// Hence the motivation of adding HTTP_JSON_V1 as the default is to avoid a breaking change when
        /// user upgrading Envoy with this change. Furthermore, we also immediately deprecate this field,
        /// since in Zipkin realm this v1 version is considered to be not preferable anymore.]
        DeprecatedAndUnavailableDoNotUse = 0,
        /// Zipkin API v2, JSON over HTTP.
        HttpJson = 1,
        /// Zipkin API v2, protobuf over HTTP.
        HttpProto = 2,
        /// \[#not-implemented-hide:\]
        Grpc = 3,
    }
    impl CollectorEndpointVersion {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::DeprecatedAndUnavailableDoNotUse => {
                    "DEPRECATED_AND_UNAVAILABLE_DO_NOT_USE"
                }
                Self::HttpJson => "HTTP_JSON",
                Self::HttpProto => "HTTP_PROTO",
                Self::Grpc => "GRPC",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DEPRECATED_AND_UNAVAILABLE_DO_NOT_USE" => {
                    Some(Self::DeprecatedAndUnavailableDoNotUse)
                }
                "HTTP_JSON" => Some(Self::HttpJson),
                "HTTP_PROTO" => Some(Self::HttpProto),
                "GRPC" => Some(Self::Grpc),
                _ => None,
            }
        }
    }
}
impl ::prost::Name for ZipkinConfig {
    const NAME: &'static str = "ZipkinConfig";
    const PACKAGE: &'static str = "envoy.config.trace.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.config.trace.v3.ZipkinConfig".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.config.trace.v3.ZipkinConfig".into()
    }
}
/// \[#extension: envoy.tracers.xray\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct XRayConfig {
    /// The UDP endpoint of the X-Ray Daemon where the spans will be sent.
    /// If this value is not set, the default value of 127.0.0.1:2000 will be used.
    #[prost(message, optional, tag = "1")]
    pub daemon_endpoint: ::core::option::Option<super::super::core::v3::SocketAddress>,
    /// The name of the X-Ray segment.
    #[prost(string, tag = "2")]
    pub segment_name: ::prost::alloc::string::String,
    /// The location of a local custom sampling rules JSON file.
    /// For an example of the sampling rules see:
    /// `X-Ray SDK documentation
    /// <<https://docs.aws.amazon.com/xray/latest/devguide/xray-sdk-go-configuration.html#xray-sdk-go-configuration-sampling>`_>
    #[prost(message, optional, tag = "3")]
    pub sampling_rule_manifest: ::core::option::Option<
        super::super::core::v3::DataSource,
    >,
    /// Optional custom fields to be added to each trace segment.
    /// see: `X-Ray Segment Document documentation
    /// <<https://docs.aws.amazon.com/xray/latest/devguide/xray-api-segmentdocuments.html>`__>
    #[prost(message, optional, tag = "4")]
    pub segment_fields: ::core::option::Option<x_ray_config::SegmentFields>,
}
/// Nested message and enum types in `XRayConfig`.
pub mod x_ray_config {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SegmentFields {
        /// The type of AWS resource, e.g. "AWS::AppMesh::Proxy".
        #[prost(string, tag = "1")]
        pub origin: ::prost::alloc::string::String,
        /// AWS resource metadata dictionary.
        /// See: `X-Ray Segment Document documentation <<https://docs.aws.amazon.com/xray/latest/devguide/xray-api-segmentdocuments.html#api-segmentdocuments-aws>`__>
        #[prost(message, optional, tag = "2")]
        pub aws: ::core::option::Option<
            super::super::super::super::super::google::protobuf::Struct,
        >,
    }
    impl ::prost::Name for SegmentFields {
        const NAME: &'static str = "SegmentFields";
        const PACKAGE: &'static str = "envoy.config.trace.v3";
        fn full_name() -> ::prost::alloc::string::String {
            "envoy.config.trace.v3.XRayConfig.SegmentFields".into()
        }
        fn type_url() -> ::prost::alloc::string::String {
            "type.googleapis.com/envoy.config.trace.v3.XRayConfig.SegmentFields".into()
        }
    }
}
impl ::prost::Name for XRayConfig {
    const NAME: &'static str = "XRayConfig";
    const PACKAGE: &'static str = "envoy.config.trace.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.config.trace.v3.XRayConfig".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.config.trace.v3.XRayConfig".into()
    }
}
