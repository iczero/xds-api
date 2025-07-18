// This file is @generated by prost-build.
/// Match an IP against a repeated CIDR range. This matcher is intended to be
/// used in other matchers, for example in the filter state matcher to match a
/// filter state object as an IP.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressMatcher {
    #[prost(message, repeated, tag = "1")]
    pub ranges: ::prost::alloc::vec::Vec<
        super::super::super::super::xds::core::v3::CidrRange,
    >,
}
impl ::prost::Name for AddressMatcher {
    const NAME: &'static str = "AddressMatcher";
    const PACKAGE: &'static str = "envoy.type.matcher.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.type.matcher.v3.AddressMatcher".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.type.matcher.v3.AddressMatcher".into()
    }
}
/// A regex matcher designed for safety when used with untrusted input.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegexMatcher {
    /// The regex match string. The string must be supported by the configured engine. The regex is matched
    /// against the full string, not as a partial match.
    #[prost(string, tag = "2")]
    pub regex: ::prost::alloc::string::String,
    #[prost(oneof = "regex_matcher::EngineType", tags = "1")]
    pub engine_type: ::core::option::Option<regex_matcher::EngineType>,
}
/// Nested message and enum types in `RegexMatcher`.
pub mod regex_matcher {
    /// Google's `RE2 <<https://github.com/google/re2>`_> regex engine. The regex string must adhere to
    /// the documented `syntax <<https://github.com/google/re2/wiki/Syntax>`_.> The engine is designed
    /// to complete execution in linear time as well as limit the amount of memory used.
    ///
    /// Envoy supports program size checking via runtime. The runtime keys ``re2.max_program_size.error_level``
    /// and ``re2.max_program_size.warn_level`` can be set to integers as the maximum program size or
    /// complexity that a compiled regex can have before an exception is thrown or a warning is
    /// logged, respectively. ``re2.max_program_size.error_level`` defaults to 100, and
    /// ``re2.max_program_size.warn_level`` has no default if unset (will not check/log a warning).
    ///
    /// Envoy emits two stats for tracking the program size of regexes: the histogram ``re2.program_size``,
    /// which records the program size, and the counter ``re2.exceeded_warn_level``, which is incremented
    /// each time the program size exceeds the warn level threshold.
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct GoogleRe2 {
        /// This field controls the RE2 "program size" which is a rough estimate of how complex a
        /// compiled regex is to evaluate. A regex that has a program size greater than the configured
        /// value will fail to compile. In this case, the configured max program size can be increased
        /// or the regex can be simplified. If not specified, the default is 100.
        ///
        /// This field is deprecated; regexp validation should be performed on the management server
        /// instead of being done by each individual client.
        ///
        /// .. note::
        ///
        ///   Although this field is deprecated, the program size will still be checked against the
        ///   global ``re2.max_program_size.error_level`` runtime value.
        ///
        #[deprecated]
        #[prost(message, optional, tag = "1")]
        pub max_program_size: ::core::option::Option<
            super::super::super::super::super::google::protobuf::UInt32Value,
        >,
    }
    impl ::prost::Name for GoogleRe2 {
        const NAME: &'static str = "GoogleRE2";
        const PACKAGE: &'static str = "envoy.type.matcher.v3";
        fn full_name() -> ::prost::alloc::string::String {
            "envoy.type.matcher.v3.RegexMatcher.GoogleRE2".into()
        }
        fn type_url() -> ::prost::alloc::string::String {
            "type.googleapis.com/envoy.type.matcher.v3.RegexMatcher.GoogleRE2".into()
        }
    }
    #[derive(Clone, Copy, PartialEq, ::prost::Oneof)]
    pub enum EngineType {
        /// Google's RE2 regex engine.
        #[prost(message, tag = "1")]
        GoogleRe2(GoogleRe2),
    }
}
impl ::prost::Name for RegexMatcher {
    const NAME: &'static str = "RegexMatcher";
    const PACKAGE: &'static str = "envoy.type.matcher.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.type.matcher.v3.RegexMatcher".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.type.matcher.v3.RegexMatcher".into()
    }
}
/// Describes how to match a string and then produce a new string using a regular
/// expression and a substitution string.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegexMatchAndSubstitute {
    /// The regular expression used to find portions of a string (hereafter called
    /// the "subject string") that should be replaced. When a new string is
    /// produced during the substitution operation, the new string is initially
    /// the same as the subject string, but then all matches in the subject string
    /// are replaced by the substitution string. If replacing all matches isn't
    /// desired, regular expression anchors can be used to ensure a single match,
    /// so as to replace just one occurrence of a pattern. Capture groups can be
    /// used in the pattern to extract portions of the subject string, and then
    /// referenced in the substitution string.
    #[prost(message, optional, tag = "1")]
    pub pattern: ::core::option::Option<RegexMatcher>,
    /// The string that should be substituted into matching portions of the
    /// subject string during a substitution operation to produce a new string.
    /// Capture groups in the pattern can be referenced in the substitution
    /// string. Note, however, that the syntax for referring to capture groups is
    /// defined by the chosen regular expression engine. Google's `RE2
    /// <<https://github.com/google/re2>`_> regular expression engine uses a
    /// backslash followed by the capture group number to denote a numbered
    /// capture group. E.g., ``\1`` refers to capture group 1, and ``\2`` refers
    /// to capture group 2.
    #[prost(string, tag = "2")]
    pub substitution: ::prost::alloc::string::String,
}
impl ::prost::Name for RegexMatchAndSubstitute {
    const NAME: &'static str = "RegexMatchAndSubstitute";
    const PACKAGE: &'static str = "envoy.type.matcher.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.type.matcher.v3.RegexMatchAndSubstitute".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.type.matcher.v3.RegexMatchAndSubstitute".into()
    }
}
/// Specifies the way to match a string.
/// \[#next-free-field: 9\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringMatcher {
    /// If ``true``, indicates the exact/prefix/suffix/contains matching should be case insensitive. This
    /// has no effect for the ``safe_regex`` match.
    /// For example, the matcher ``data`` will match both input string ``Data`` and ``data`` if this option
    /// is set to ``true``.
    #[prost(bool, tag = "6")]
    pub ignore_case: bool,
    #[prost(oneof = "string_matcher::MatchPattern", tags = "1, 2, 3, 5, 7, 8")]
    pub match_pattern: ::core::option::Option<string_matcher::MatchPattern>,
}
/// Nested message and enum types in `StringMatcher`.
pub mod string_matcher {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MatchPattern {
        /// The input string must match exactly the string specified here.
        ///
        /// Examples:
        ///
        /// * ``abc`` only matches the value ``abc``.
        #[prost(string, tag = "1")]
        Exact(::prost::alloc::string::String),
        /// The input string must have the prefix specified here.
        ///
        /// .. note::
        ///
        ///   Empty prefix match is not allowed, please use ``safe_regex`` instead.
        ///
        /// Examples:
        ///
        /// * ``abc`` matches the value ``abc.xyz``
        #[prost(string, tag = "2")]
        Prefix(::prost::alloc::string::String),
        /// The input string must have the suffix specified here.
        ///
        /// .. note::
        ///
        ///   Empty suffix match is not allowed, please use ``safe_regex`` instead.
        ///
        /// Examples:
        ///
        /// * ``abc`` matches the value ``xyz.abc``
        #[prost(string, tag = "3")]
        Suffix(::prost::alloc::string::String),
        /// The input string must match the regular expression specified here.
        #[prost(message, tag = "5")]
        SafeRegex(super::RegexMatcher),
        /// The input string must have the substring specified here.
        ///
        /// .. note::
        ///
        ///   Empty contains match is not allowed, please use ``safe_regex`` instead.
        ///
        /// Examples:
        ///
        /// * ``abc`` matches the value ``xyz.abc.def``
        #[prost(string, tag = "7")]
        Contains(::prost::alloc::string::String),
        /// Use an extension as the matcher type.
        /// \[#extension-category: envoy.string_matcher\]
        #[prost(message, tag = "8")]
        Custom(super::super::super::super::super::xds::core::v3::TypedExtensionConfig),
    }
}
impl ::prost::Name for StringMatcher {
    const NAME: &'static str = "StringMatcher";
    const PACKAGE: &'static str = "envoy.type.matcher.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.type.matcher.v3.StringMatcher".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.type.matcher.v3.StringMatcher".into()
    }
}
/// Specifies a list of ways to match a string.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStringMatcher {
    #[prost(message, repeated, tag = "1")]
    pub patterns: ::prost::alloc::vec::Vec<StringMatcher>,
}
impl ::prost::Name for ListStringMatcher {
    const NAME: &'static str = "ListStringMatcher";
    const PACKAGE: &'static str = "envoy.type.matcher.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.type.matcher.v3.ListStringMatcher".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.type.matcher.v3.ListStringMatcher".into()
    }
}
/// FilterStateMatcher provides a general interface for matching the filter state objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterStateMatcher {
    /// The filter state key to retrieve the object.
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(oneof = "filter_state_matcher::Matcher", tags = "2, 3")]
    pub matcher: ::core::option::Option<filter_state_matcher::Matcher>,
}
/// Nested message and enum types in `FilterStateMatcher`.
pub mod filter_state_matcher {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Matcher {
        /// Matches the filter state object as a string value.
        #[prost(message, tag = "2")]
        StringMatch(super::StringMatcher),
        /// Matches the filter state object as a ip Instance.
        #[prost(message, tag = "3")]
        AddressMatch(super::AddressMatcher),
    }
}
impl ::prost::Name for FilterStateMatcher {
    const NAME: &'static str = "FilterStateMatcher";
    const PACKAGE: &'static str = "envoy.type.matcher.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.type.matcher.v3.FilterStateMatcher".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.type.matcher.v3.FilterStateMatcher".into()
    }
}
/// Match input indicates that matching should be done on a specific request header.
/// The resulting input string will be all headers for the given key joined by a comma,
/// e.g. if the request contains two 'foo' headers with value 'bar' and 'baz', the input
/// string will be 'bar,baz'.
/// \[#comment:TODO(snowp): Link to unified matching docs.\]
/// \[#extension: envoy.matching.inputs.request_headers\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpRequestHeaderMatchInput {
    /// The request header to match on.
    #[prost(string, tag = "1")]
    pub header_name: ::prost::alloc::string::String,
}
impl ::prost::Name for HttpRequestHeaderMatchInput {
    const NAME: &'static str = "HttpRequestHeaderMatchInput";
    const PACKAGE: &'static str = "envoy.type.matcher.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.type.matcher.v3.HttpRequestHeaderMatchInput".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.type.matcher.v3.HttpRequestHeaderMatchInput".into()
    }
}
/// Match input indicates that matching should be done on a specific request trailer.
/// The resulting input string will be all headers for the given key joined by a comma,
/// e.g. if the request contains two 'foo' headers with value 'bar' and 'baz', the input
/// string will be 'bar,baz'.
/// \[#comment:TODO(snowp): Link to unified matching docs.\]
/// \[#extension: envoy.matching.inputs.request_trailers\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpRequestTrailerMatchInput {
    /// The request trailer to match on.
    #[prost(string, tag = "1")]
    pub header_name: ::prost::alloc::string::String,
}
impl ::prost::Name for HttpRequestTrailerMatchInput {
    const NAME: &'static str = "HttpRequestTrailerMatchInput";
    const PACKAGE: &'static str = "envoy.type.matcher.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.type.matcher.v3.HttpRequestTrailerMatchInput".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.type.matcher.v3.HttpRequestTrailerMatchInput".into()
    }
}
/// Match input indicating that matching should be done on a specific response header.
/// The resulting input string will be all headers for the given key joined by a comma,
/// e.g. if the response contains two 'foo' headers with value 'bar' and 'baz', the input
/// string will be 'bar,baz'.
/// \[#comment:TODO(snowp): Link to unified matching docs.\]
/// \[#extension: envoy.matching.inputs.response_headers\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpResponseHeaderMatchInput {
    /// The response header to match on.
    #[prost(string, tag = "1")]
    pub header_name: ::prost::alloc::string::String,
}
impl ::prost::Name for HttpResponseHeaderMatchInput {
    const NAME: &'static str = "HttpResponseHeaderMatchInput";
    const PACKAGE: &'static str = "envoy.type.matcher.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.type.matcher.v3.HttpResponseHeaderMatchInput".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.type.matcher.v3.HttpResponseHeaderMatchInput".into()
    }
}
/// Match input indicates that matching should be done on a specific response trailer.
/// The resulting input string will be all headers for the given key joined by a comma,
/// e.g. if the request contains two 'foo' headers with value 'bar' and 'baz', the input
/// string will be 'bar,baz'.
/// \[#comment:TODO(snowp): Link to unified matching docs.\]
/// \[#extension: envoy.matching.inputs.response_trailers\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpResponseTrailerMatchInput {
    /// The response trailer to match on.
    #[prost(string, tag = "1")]
    pub header_name: ::prost::alloc::string::String,
}
impl ::prost::Name for HttpResponseTrailerMatchInput {
    const NAME: &'static str = "HttpResponseTrailerMatchInput";
    const PACKAGE: &'static str = "envoy.type.matcher.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.type.matcher.v3.HttpResponseTrailerMatchInput".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.type.matcher.v3.HttpResponseTrailerMatchInput".into()
    }
}
/// Match input indicates that matching should be done on a specific query parameter.
/// The resulting input string will be the first query parameter for the value
/// 'query_param'.
/// \[#extension: envoy.matching.inputs.query_params\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpRequestQueryParamMatchInput {
    /// The query parameter to match on.
    #[prost(string, tag = "1")]
    pub query_param: ::prost::alloc::string::String,
}
impl ::prost::Name for HttpRequestQueryParamMatchInput {
    const NAME: &'static str = "HttpRequestQueryParamMatchInput";
    const PACKAGE: &'static str = "envoy.type.matcher.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.type.matcher.v3.HttpRequestQueryParamMatchInput".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.type.matcher.v3.HttpRequestQueryParamMatchInput"
            .into()
    }
}
/// Specifies the way to match a double value.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct DoubleMatcher {
    #[prost(oneof = "double_matcher::MatchPattern", tags = "1, 2")]
    pub match_pattern: ::core::option::Option<double_matcher::MatchPattern>,
}
/// Nested message and enum types in `DoubleMatcher`.
pub mod double_matcher {
    #[derive(Clone, Copy, PartialEq, ::prost::Oneof)]
    pub enum MatchPattern {
        /// If specified, the input double value must be in the range specified here.
        /// Note: The range is using half-open interval semantics [start, end).
        #[prost(message, tag = "1")]
        Range(super::super::super::v3::DoubleRange),
        /// If specified, the input double value must be equal to the value specified here.
        #[prost(double, tag = "2")]
        Exact(f64),
    }
}
impl ::prost::Name for DoubleMatcher {
    const NAME: &'static str = "DoubleMatcher";
    const PACKAGE: &'static str = "envoy.type.matcher.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.type.matcher.v3.DoubleMatcher".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.type.matcher.v3.DoubleMatcher".into()
    }
}
/// Specifies the way to match a ProtobufWkt::Value. Primitive values and ListValue are supported.
/// StructValue is not supported and is always not matched.
/// \[#next-free-field: 8\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValueMatcher {
    /// Specifies how to match a value.
    #[prost(oneof = "value_matcher::MatchPattern", tags = "1, 2, 3, 4, 5, 6, 7")]
    pub match_pattern: ::core::option::Option<value_matcher::MatchPattern>,
}
/// Nested message and enum types in `ValueMatcher`.
pub mod value_matcher {
    /// NullMatch is an empty message to specify a null value.
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct NullMatch {}
    impl ::prost::Name for NullMatch {
        const NAME: &'static str = "NullMatch";
        const PACKAGE: &'static str = "envoy.type.matcher.v3";
        fn full_name() -> ::prost::alloc::string::String {
            "envoy.type.matcher.v3.ValueMatcher.NullMatch".into()
        }
        fn type_url() -> ::prost::alloc::string::String {
            "type.googleapis.com/envoy.type.matcher.v3.ValueMatcher.NullMatch".into()
        }
    }
    /// Specifies how to match a value.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MatchPattern {
        /// If specified, a match occurs if and only if the target value is a NullValue.
        #[prost(message, tag = "1")]
        NullMatch(NullMatch),
        /// If specified, a match occurs if and only if the target value is a double value and is
        /// matched to this field.
        #[prost(message, tag = "2")]
        DoubleMatch(super::DoubleMatcher),
        /// If specified, a match occurs if and only if the target value is a string value and is
        /// matched to this field.
        #[prost(message, tag = "3")]
        StringMatch(super::StringMatcher),
        /// If specified, a match occurs if and only if the target value is a bool value and is equal
        /// to this field.
        #[prost(bool, tag = "4")]
        BoolMatch(bool),
        /// If specified, value match will be performed based on whether the path is referring to a
        /// valid primitive value in the metadata. If the path is referring to a non-primitive value,
        /// the result is always not matched.
        #[prost(bool, tag = "5")]
        PresentMatch(bool),
        /// If specified, a match occurs if and only if the target value is a list value and
        /// is matched to this field.
        #[prost(message, tag = "6")]
        ListMatch(::prost::alloc::boxed::Box<super::ListMatcher>),
        /// If specified, a match occurs if and only if any of the alternatives in the match accept the value.
        #[prost(message, tag = "7")]
        OrMatch(super::OrMatcher),
    }
}
impl ::prost::Name for ValueMatcher {
    const NAME: &'static str = "ValueMatcher";
    const PACKAGE: &'static str = "envoy.type.matcher.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.type.matcher.v3.ValueMatcher".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.type.matcher.v3.ValueMatcher".into()
    }
}
/// Specifies the way to match a list value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMatcher {
    #[prost(oneof = "list_matcher::MatchPattern", tags = "1")]
    pub match_pattern: ::core::option::Option<list_matcher::MatchPattern>,
}
/// Nested message and enum types in `ListMatcher`.
pub mod list_matcher {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MatchPattern {
        /// If specified, at least one of the values in the list must match the value specified.
        #[prost(message, tag = "1")]
        OneOf(::prost::alloc::boxed::Box<super::ValueMatcher>),
    }
}
impl ::prost::Name for ListMatcher {
    const NAME: &'static str = "ListMatcher";
    const PACKAGE: &'static str = "envoy.type.matcher.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.type.matcher.v3.ListMatcher".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.type.matcher.v3.ListMatcher".into()
    }
}
/// Specifies a list of alternatives for the match.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrMatcher {
    #[prost(message, repeated, tag = "1")]
    pub value_matchers: ::prost::alloc::vec::Vec<ValueMatcher>,
}
impl ::prost::Name for OrMatcher {
    const NAME: &'static str = "OrMatcher";
    const PACKAGE: &'static str = "envoy.type.matcher.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.type.matcher.v3.OrMatcher".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.type.matcher.v3.OrMatcher".into()
    }
}
/// \[#next-major-version: MetadataMatcher should use StructMatcher\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetadataMatcher {
    /// The filter name to retrieve the ``Struct`` from the ``Metadata``.
    #[prost(string, tag = "1")]
    pub filter: ::prost::alloc::string::String,
    /// The path to retrieve the ``Value`` from the ``Struct``.
    #[prost(message, repeated, tag = "2")]
    pub path: ::prost::alloc::vec::Vec<metadata_matcher::PathSegment>,
    /// The ``MetadataMatcher`` is matched if the value retrieved by path is matched to this value.
    #[prost(message, optional, tag = "3")]
    pub value: ::core::option::Option<ValueMatcher>,
    /// If true, the match result will be inverted.
    #[prost(bool, tag = "4")]
    pub invert: bool,
}
/// Nested message and enum types in `MetadataMatcher`.
pub mod metadata_matcher {
    /// Specifies the segment in a path to retrieve value from ``Metadata``.
    ///
    /// .. note::
    ///    Currently it's not supported to retrieve a value from a list in ``Metadata``. This means that
    ///    if the segment key refers to a list, it has to be the last segment in a path.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PathSegment {
        #[prost(oneof = "path_segment::Segment", tags = "1")]
        pub segment: ::core::option::Option<path_segment::Segment>,
    }
    /// Nested message and enum types in `PathSegment`.
    pub mod path_segment {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Segment {
            /// If specified, use the key to retrieve the value in a ``Struct``.
            #[prost(string, tag = "1")]
            Key(::prost::alloc::string::String),
        }
    }
    impl ::prost::Name for PathSegment {
        const NAME: &'static str = "PathSegment";
        const PACKAGE: &'static str = "envoy.type.matcher.v3";
        fn full_name() -> ::prost::alloc::string::String {
            "envoy.type.matcher.v3.MetadataMatcher.PathSegment".into()
        }
        fn type_url() -> ::prost::alloc::string::String {
            "type.googleapis.com/envoy.type.matcher.v3.MetadataMatcher.PathSegment"
                .into()
        }
    }
}
impl ::prost::Name for MetadataMatcher {
    const NAME: &'static str = "MetadataMatcher";
    const PACKAGE: &'static str = "envoy.type.matcher.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.type.matcher.v3.MetadataMatcher".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.type.matcher.v3.MetadataMatcher".into()
    }
}
/// StructMatcher provides a general interface to check if a given value is matched in
/// google.protobuf.Struct. It uses ``path`` to retrieve the value
/// from the struct and then check if it's matched to the specified value.
///
/// For example, for the following Struct:
///
/// .. code-block:: yaml
///
///         fields:
///           a:
///             struct_value:
///               fields:
///                 b:
///                   struct_value:
///                     fields:
///                       c:
///                         string_value: pro
///                 t:
///                   list_value:
///                     values:
///                       - string_value: m
///                       - string_value: n
///
/// The following MetadataMatcher is matched as the path \[a, b, c\] will retrieve a string value "pro"
/// from the Metadata which is matched to the specified prefix match.
///
/// .. code-block:: yaml
///
///     path:
///     - key: a
///     - key: b
///     - key: c
///     value:
///       string_match:
///         prefix: pr
///
/// The following StructMatcher is matched as the code will match one of the string values in the
/// list at the path \[a, t\].
///
/// .. code-block:: yaml
///
///     path:
///     - key: a
///     - key: t
///     value:
///       list_match:
///         one_of:
///           string_match:
///             exact: m
///
/// An example use of StructMatcher is to match metadata in envoy.v*.core.Node.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StructMatcher {
    /// The path to retrieve the Value from the Struct.
    #[prost(message, repeated, tag = "2")]
    pub path: ::prost::alloc::vec::Vec<struct_matcher::PathSegment>,
    /// The StructMatcher is matched if the value retrieved by path is matched to this value.
    #[prost(message, optional, tag = "3")]
    pub value: ::core::option::Option<ValueMatcher>,
}
/// Nested message and enum types in `StructMatcher`.
pub mod struct_matcher {
    /// Specifies the segment in a path to retrieve value from Struct.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PathSegment {
        #[prost(oneof = "path_segment::Segment", tags = "1")]
        pub segment: ::core::option::Option<path_segment::Segment>,
    }
    /// Nested message and enum types in `PathSegment`.
    pub mod path_segment {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Segment {
            /// If specified, use the key to retrieve the value in a Struct.
            #[prost(string, tag = "1")]
            Key(::prost::alloc::string::String),
        }
    }
    impl ::prost::Name for PathSegment {
        const NAME: &'static str = "PathSegment";
        const PACKAGE: &'static str = "envoy.type.matcher.v3";
        fn full_name() -> ::prost::alloc::string::String {
            "envoy.type.matcher.v3.StructMatcher.PathSegment".into()
        }
        fn type_url() -> ::prost::alloc::string::String {
            "type.googleapis.com/envoy.type.matcher.v3.StructMatcher.PathSegment".into()
        }
    }
}
impl ::prost::Name for StructMatcher {
    const NAME: &'static str = "StructMatcher";
    const PACKAGE: &'static str = "envoy.type.matcher.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.type.matcher.v3.StructMatcher".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.type.matcher.v3.StructMatcher".into()
    }
}
/// Specifies the way to match a Node.
/// The match follows AND semantics.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeMatcher {
    /// Specifies match criteria on the node id.
    #[prost(message, optional, tag = "1")]
    pub node_id: ::core::option::Option<StringMatcher>,
    /// Specifies match criteria on the node metadata.
    #[prost(message, repeated, tag = "2")]
    pub node_metadatas: ::prost::alloc::vec::Vec<StructMatcher>,
}
impl ::prost::Name for NodeMatcher {
    const NAME: &'static str = "NodeMatcher";
    const PACKAGE: &'static str = "envoy.type.matcher.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.type.matcher.v3.NodeMatcher".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.type.matcher.v3.NodeMatcher".into()
    }
}
/// Specifies the way to match a path on HTTP request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PathMatcher {
    #[prost(oneof = "path_matcher::Rule", tags = "1")]
    pub rule: ::core::option::Option<path_matcher::Rule>,
}
/// Nested message and enum types in `PathMatcher`.
pub mod path_matcher {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Rule {
        /// The ``path`` must match the URL path portion of the :path header. The query and fragment
        /// string (if present) are removed in the URL path portion.
        /// For example, the path ``/data`` will match the ``:path`` header ``/data#fragment?param=value``.
        #[prost(message, tag = "1")]
        Path(super::StringMatcher),
    }
}
impl ::prost::Name for PathMatcher {
    const NAME: &'static str = "PathMatcher";
    const PACKAGE: &'static str = "envoy.type.matcher.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.type.matcher.v3.PathMatcher".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.type.matcher.v3.PathMatcher".into()
    }
}
/// Match input indicates that matching should be done on the response status
/// code.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct HttpResponseStatusCodeMatchInput {}
impl ::prost::Name for HttpResponseStatusCodeMatchInput {
    const NAME: &'static str = "HttpResponseStatusCodeMatchInput";
    const PACKAGE: &'static str = "envoy.type.matcher.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.type.matcher.v3.HttpResponseStatusCodeMatchInput".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.type.matcher.v3.HttpResponseStatusCodeMatchInput"
            .into()
    }
}
/// Match input indicates that the matching should be done on the class of the
/// response status code. For eg: 1xx, 2xx, 3xx, 4xx or 5xx.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct HttpResponseStatusCodeClassMatchInput {}
impl ::prost::Name for HttpResponseStatusCodeClassMatchInput {
    const NAME: &'static str = "HttpResponseStatusCodeClassMatchInput";
    const PACKAGE: &'static str = "envoy.type.matcher.v3";
    fn full_name() -> ::prost::alloc::string::String {
        "envoy.type.matcher.v3.HttpResponseStatusCodeClassMatchInput".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "type.googleapis.com/envoy.type.matcher.v3.HttpResponseStatusCodeClassMatchInput"
            .into()
    }
}
