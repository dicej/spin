#![deny(warnings)]

#[doc(hidden)]
pub use Body as __with_name0;
#[doc(hidden)]
pub use Fields as __with_name1;
#[doc(hidden)]
pub use Request as __with_name2;
#[doc(hidden)]
pub use RequestOptions as __with_name3;
#[doc(hidden)]
pub use Response as __with_name4;
pub mod wasi {
    pub mod http {

        #[allow(clippy::all)]
        pub mod types {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::{anyhow, Box};

            pub type Duration = u64;
            const _: () = {
                assert!(8 == <Duration as wasmtime::component::ComponentType>::SIZE32);
                assert!(8 == <Duration as wasmtime::component::ComponentType>::ALIGN32);
            };
            /// This type corresponds to HTTP standard Methods.
            #[derive(
                wasmtime::component::ComponentType,
                wasmtime::component::Lift,
                wasmtime::component::Lower,
            )]
            #[component(variant)]
            #[derive(Clone)]
            pub enum Method {
                #[component(name = "get")]
                Get,
                #[component(name = "head")]
                Head,
                #[component(name = "post")]
                Post,
                #[component(name = "put")]
                Put,
                #[component(name = "delete")]
                Delete,
                #[component(name = "connect")]
                Connect,
                #[component(name = "options")]
                Options,
                #[component(name = "trace")]
                Trace,
                #[component(name = "patch")]
                Patch,
                #[component(name = "other")]
                Other(wasmtime::component::__internal::String),
            }
            impl core::fmt::Debug for Method {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        Method::Get => f.debug_tuple("Method::Get").finish(),
                        Method::Head => f.debug_tuple("Method::Head").finish(),
                        Method::Post => f.debug_tuple("Method::Post").finish(),
                        Method::Put => f.debug_tuple("Method::Put").finish(),
                        Method::Delete => f.debug_tuple("Method::Delete").finish(),
                        Method::Connect => f.debug_tuple("Method::Connect").finish(),
                        Method::Options => f.debug_tuple("Method::Options").finish(),
                        Method::Trace => f.debug_tuple("Method::Trace").finish(),
                        Method::Patch => f.debug_tuple("Method::Patch").finish(),
                        Method::Other(e) => f.debug_tuple("Method::Other").field(e).finish(),
                    }
                }
            }
            const _: () = {
                assert!(12 == <Method as wasmtime::component::ComponentType>::SIZE32);
                assert!(4 == <Method as wasmtime::component::ComponentType>::ALIGN32);
            };
            /// This type corresponds to HTTP standard Related Schemes.
            #[derive(
                wasmtime::component::ComponentType,
                wasmtime::component::Lift,
                wasmtime::component::Lower,
            )]
            #[component(variant)]
            #[derive(Clone)]
            pub enum Scheme {
                #[component(name = "HTTP")]
                Http,
                #[component(name = "HTTPS")]
                Https,
                #[component(name = "other")]
                Other(wasmtime::component::__internal::String),
            }
            impl core::fmt::Debug for Scheme {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        Scheme::Http => f.debug_tuple("Scheme::Http").finish(),
                        Scheme::Https => f.debug_tuple("Scheme::Https").finish(),
                        Scheme::Other(e) => f.debug_tuple("Scheme::Other").field(e).finish(),
                    }
                }
            }
            const _: () = {
                assert!(12 == <Scheme as wasmtime::component::ComponentType>::SIZE32);
                assert!(4 == <Scheme as wasmtime::component::ComponentType>::ALIGN32);
            };
            /// Defines the case payload type for `DNS-error` above:
            #[derive(
                wasmtime::component::ComponentType,
                wasmtime::component::Lift,
                wasmtime::component::Lower,
            )]
            #[component(record)]
            #[derive(Clone)]
            pub struct DnsErrorPayload {
                #[component(name = "rcode")]
                pub rcode: Option<wasmtime::component::__internal::String>,
                #[component(name = "info-code")]
                pub info_code: Option<u16>,
            }
            impl core::fmt::Debug for DnsErrorPayload {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("DnsErrorPayload")
                        .field("rcode", &self.rcode)
                        .field("info-code", &self.info_code)
                        .finish()
                }
            }
            const _: () = {
                assert!(16 == <DnsErrorPayload as wasmtime::component::ComponentType>::SIZE32);
                assert!(4 == <DnsErrorPayload as wasmtime::component::ComponentType>::ALIGN32);
            };
            /// Defines the case payload type for `TLS-alert-received` above:
            #[derive(
                wasmtime::component::ComponentType,
                wasmtime::component::Lift,
                wasmtime::component::Lower,
            )]
            #[component(record)]
            #[derive(Clone)]
            pub struct TlsAlertReceivedPayload {
                #[component(name = "alert-id")]
                pub alert_id: Option<u8>,
                #[component(name = "alert-message")]
                pub alert_message: Option<wasmtime::component::__internal::String>,
            }
            impl core::fmt::Debug for TlsAlertReceivedPayload {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("TlsAlertReceivedPayload")
                        .field("alert-id", &self.alert_id)
                        .field("alert-message", &self.alert_message)
                        .finish()
                }
            }
            const _: () = {
                assert!(
                    16 == <TlsAlertReceivedPayload as wasmtime::component::ComponentType>::SIZE32
                );
                assert!(
                    4 == <TlsAlertReceivedPayload as wasmtime::component::ComponentType>::ALIGN32
                );
            };
            /// Defines the case payload type for `HTTP-response-{header,trailer}-size` above:
            #[derive(
                wasmtime::component::ComponentType,
                wasmtime::component::Lift,
                wasmtime::component::Lower,
            )]
            #[component(record)]
            #[derive(Clone)]
            pub struct FieldSizePayload {
                #[component(name = "field-name")]
                pub field_name: Option<wasmtime::component::__internal::String>,
                #[component(name = "field-size")]
                pub field_size: Option<u32>,
            }
            impl core::fmt::Debug for FieldSizePayload {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("FieldSizePayload")
                        .field("field-name", &self.field_name)
                        .field("field-size", &self.field_size)
                        .finish()
                }
            }
            const _: () = {
                assert!(20 == <FieldSizePayload as wasmtime::component::ComponentType>::SIZE32);
                assert!(4 == <FieldSizePayload as wasmtime::component::ComponentType>::ALIGN32);
            };
            /// These cases are inspired by the IANA HTTP Proxy Error Types:
            ///   https://www.iana.org/assignments/http-proxy-status/http-proxy-status.xhtml#table-http-proxy-error-types
            #[derive(
                wasmtime::component::ComponentType,
                wasmtime::component::Lift,
                wasmtime::component::Lower,
            )]
            #[component(variant)]
            #[derive(Clone)]
            pub enum ErrorCode {
                #[component(name = "DNS-timeout")]
                DnsTimeout,
                #[component(name = "DNS-error")]
                DnsError(DnsErrorPayload),
                #[component(name = "destination-not-found")]
                DestinationNotFound,
                #[component(name = "destination-unavailable")]
                DestinationUnavailable,
                #[component(name = "destination-IP-prohibited")]
                DestinationIpProhibited,
                #[component(name = "destination-IP-unroutable")]
                DestinationIpUnroutable,
                #[component(name = "connection-refused")]
                ConnectionRefused,
                #[component(name = "connection-terminated")]
                ConnectionTerminated,
                #[component(name = "connection-timeout")]
                ConnectionTimeout,
                #[component(name = "connection-read-timeout")]
                ConnectionReadTimeout,
                #[component(name = "connection-write-timeout")]
                ConnectionWriteTimeout,
                #[component(name = "connection-limit-reached")]
                ConnectionLimitReached,
                #[component(name = "TLS-protocol-error")]
                TlsProtocolError,
                #[component(name = "TLS-certificate-error")]
                TlsCertificateError,
                #[component(name = "TLS-alert-received")]
                TlsAlertReceived(TlsAlertReceivedPayload),
                #[component(name = "HTTP-request-denied")]
                HttpRequestDenied,
                #[component(name = "HTTP-request-length-required")]
                HttpRequestLengthRequired,
                #[component(name = "HTTP-request-body-size")]
                HttpRequestBodySize(Option<u64>),
                #[component(name = "HTTP-request-method-invalid")]
                HttpRequestMethodInvalid,
                #[component(name = "HTTP-request-URI-invalid")]
                HttpRequestUriInvalid,
                #[component(name = "HTTP-request-URI-too-long")]
                HttpRequestUriTooLong,
                #[component(name = "HTTP-request-header-section-size")]
                HttpRequestHeaderSectionSize(Option<u32>),
                #[component(name = "HTTP-request-header-size")]
                HttpRequestHeaderSize(Option<FieldSizePayload>),
                #[component(name = "HTTP-request-trailer-section-size")]
                HttpRequestTrailerSectionSize(Option<u32>),
                #[component(name = "HTTP-request-trailer-size")]
                HttpRequestTrailerSize(FieldSizePayload),
                #[component(name = "HTTP-response-incomplete")]
                HttpResponseIncomplete,
                #[component(name = "HTTP-response-header-section-size")]
                HttpResponseHeaderSectionSize(Option<u32>),
                #[component(name = "HTTP-response-header-size")]
                HttpResponseHeaderSize(FieldSizePayload),
                #[component(name = "HTTP-response-body-size")]
                HttpResponseBodySize(Option<u64>),
                #[component(name = "HTTP-response-trailer-section-size")]
                HttpResponseTrailerSectionSize(Option<u32>),
                #[component(name = "HTTP-response-trailer-size")]
                HttpResponseTrailerSize(FieldSizePayload),
                #[component(name = "HTTP-response-transfer-coding")]
                HttpResponseTransferCoding(Option<wasmtime::component::__internal::String>),
                #[component(name = "HTTP-response-content-coding")]
                HttpResponseContentCoding(Option<wasmtime::component::__internal::String>),
                #[component(name = "HTTP-response-timeout")]
                HttpResponseTimeout,
                #[component(name = "HTTP-upgrade-failed")]
                HttpUpgradeFailed,
                #[component(name = "HTTP-protocol-error")]
                HttpProtocolError,
                #[component(name = "loop-detected")]
                LoopDetected,
                #[component(name = "configuration-error")]
                ConfigurationError,
                /// This is a catch-all error for anything that doesn't fit cleanly into a
                /// more specific case. It also includes an optional string for an
                /// unstructured description of the error. Users should not depend on the
                /// string for diagnosing errors, as it's not required to be consistent
                /// between implementations.
                #[component(name = "internal-error")]
                InternalError(Option<wasmtime::component::__internal::String>),
            }
            impl core::fmt::Debug for ErrorCode {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        ErrorCode::DnsTimeout => f.debug_tuple("ErrorCode::DnsTimeout").finish(),
                        ErrorCode::DnsError(e) => {
                            f.debug_tuple("ErrorCode::DnsError").field(e).finish()
                        }
                        ErrorCode::DestinationNotFound => {
                            f.debug_tuple("ErrorCode::DestinationNotFound").finish()
                        }
                        ErrorCode::DestinationUnavailable => {
                            f.debug_tuple("ErrorCode::DestinationUnavailable").finish()
                        }
                        ErrorCode::DestinationIpProhibited => {
                            f.debug_tuple("ErrorCode::DestinationIpProhibited").finish()
                        }
                        ErrorCode::DestinationIpUnroutable => {
                            f.debug_tuple("ErrorCode::DestinationIpUnroutable").finish()
                        }
                        ErrorCode::ConnectionRefused => {
                            f.debug_tuple("ErrorCode::ConnectionRefused").finish()
                        }
                        ErrorCode::ConnectionTerminated => {
                            f.debug_tuple("ErrorCode::ConnectionTerminated").finish()
                        }
                        ErrorCode::ConnectionTimeout => {
                            f.debug_tuple("ErrorCode::ConnectionTimeout").finish()
                        }
                        ErrorCode::ConnectionReadTimeout => {
                            f.debug_tuple("ErrorCode::ConnectionReadTimeout").finish()
                        }
                        ErrorCode::ConnectionWriteTimeout => {
                            f.debug_tuple("ErrorCode::ConnectionWriteTimeout").finish()
                        }
                        ErrorCode::ConnectionLimitReached => {
                            f.debug_tuple("ErrorCode::ConnectionLimitReached").finish()
                        }
                        ErrorCode::TlsProtocolError => {
                            f.debug_tuple("ErrorCode::TlsProtocolError").finish()
                        }
                        ErrorCode::TlsCertificateError => {
                            f.debug_tuple("ErrorCode::TlsCertificateError").finish()
                        }
                        ErrorCode::TlsAlertReceived(e) => f
                            .debug_tuple("ErrorCode::TlsAlertReceived")
                            .field(e)
                            .finish(),
                        ErrorCode::HttpRequestDenied => {
                            f.debug_tuple("ErrorCode::HttpRequestDenied").finish()
                        }
                        ErrorCode::HttpRequestLengthRequired => f
                            .debug_tuple("ErrorCode::HttpRequestLengthRequired")
                            .finish(),
                        ErrorCode::HttpRequestBodySize(e) => f
                            .debug_tuple("ErrorCode::HttpRequestBodySize")
                            .field(e)
                            .finish(),
                        ErrorCode::HttpRequestMethodInvalid => f
                            .debug_tuple("ErrorCode::HttpRequestMethodInvalid")
                            .finish(),
                        ErrorCode::HttpRequestUriInvalid => {
                            f.debug_tuple("ErrorCode::HttpRequestUriInvalid").finish()
                        }
                        ErrorCode::HttpRequestUriTooLong => {
                            f.debug_tuple("ErrorCode::HttpRequestUriTooLong").finish()
                        }
                        ErrorCode::HttpRequestHeaderSectionSize(e) => f
                            .debug_tuple("ErrorCode::HttpRequestHeaderSectionSize")
                            .field(e)
                            .finish(),
                        ErrorCode::HttpRequestHeaderSize(e) => f
                            .debug_tuple("ErrorCode::HttpRequestHeaderSize")
                            .field(e)
                            .finish(),
                        ErrorCode::HttpRequestTrailerSectionSize(e) => f
                            .debug_tuple("ErrorCode::HttpRequestTrailerSectionSize")
                            .field(e)
                            .finish(),
                        ErrorCode::HttpRequestTrailerSize(e) => f
                            .debug_tuple("ErrorCode::HttpRequestTrailerSize")
                            .field(e)
                            .finish(),
                        ErrorCode::HttpResponseIncomplete => {
                            f.debug_tuple("ErrorCode::HttpResponseIncomplete").finish()
                        }
                        ErrorCode::HttpResponseHeaderSectionSize(e) => f
                            .debug_tuple("ErrorCode::HttpResponseHeaderSectionSize")
                            .field(e)
                            .finish(),
                        ErrorCode::HttpResponseHeaderSize(e) => f
                            .debug_tuple("ErrorCode::HttpResponseHeaderSize")
                            .field(e)
                            .finish(),
                        ErrorCode::HttpResponseBodySize(e) => f
                            .debug_tuple("ErrorCode::HttpResponseBodySize")
                            .field(e)
                            .finish(),
                        ErrorCode::HttpResponseTrailerSectionSize(e) => f
                            .debug_tuple("ErrorCode::HttpResponseTrailerSectionSize")
                            .field(e)
                            .finish(),
                        ErrorCode::HttpResponseTrailerSize(e) => f
                            .debug_tuple("ErrorCode::HttpResponseTrailerSize")
                            .field(e)
                            .finish(),
                        ErrorCode::HttpResponseTransferCoding(e) => f
                            .debug_tuple("ErrorCode::HttpResponseTransferCoding")
                            .field(e)
                            .finish(),
                        ErrorCode::HttpResponseContentCoding(e) => f
                            .debug_tuple("ErrorCode::HttpResponseContentCoding")
                            .field(e)
                            .finish(),
                        ErrorCode::HttpResponseTimeout => {
                            f.debug_tuple("ErrorCode::HttpResponseTimeout").finish()
                        }
                        ErrorCode::HttpUpgradeFailed => {
                            f.debug_tuple("ErrorCode::HttpUpgradeFailed").finish()
                        }
                        ErrorCode::HttpProtocolError => {
                            f.debug_tuple("ErrorCode::HttpProtocolError").finish()
                        }
                        ErrorCode::LoopDetected => {
                            f.debug_tuple("ErrorCode::LoopDetected").finish()
                        }
                        ErrorCode::ConfigurationError => {
                            f.debug_tuple("ErrorCode::ConfigurationError").finish()
                        }
                        ErrorCode::InternalError(e) => {
                            f.debug_tuple("ErrorCode::InternalError").field(e).finish()
                        }
                    }
                }
            }
            impl core::fmt::Display for ErrorCode {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    write!(f, "{:?}", self)
                }
            }
            impl core::error::Error for ErrorCode {}
            const _: () = {
                assert!(32 == <ErrorCode as wasmtime::component::ComponentType>::SIZE32);
                assert!(8 == <ErrorCode as wasmtime::component::ComponentType>::ALIGN32);
            };
            /// This type enumerates the different kinds of errors that may occur when
            /// setting or appending to a `fields` resource.
            #[derive(
                wasmtime::component::ComponentType,
                wasmtime::component::Lift,
                wasmtime::component::Lower,
            )]
            #[component(variant)]
            #[derive(Clone, Copy)]
            pub enum HeaderError {
                /// This error indicates that a `field-key` or `field-value` was
                /// syntactically invalid when used with an operation that sets headers in a
                /// `fields`.
                #[component(name = "invalid-syntax")]
                InvalidSyntax,
                /// This error indicates that a forbidden `field-key` was used when trying
                /// to set a header in a `fields`.
                #[component(name = "forbidden")]
                Forbidden,
                /// This error indicates that the operation on the `fields` was not
                /// permitted because the fields are immutable.
                #[component(name = "immutable")]
                Immutable,
            }
            impl core::fmt::Debug for HeaderError {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        HeaderError::InvalidSyntax => {
                            f.debug_tuple("HeaderError::InvalidSyntax").finish()
                        }
                        HeaderError::Forbidden => f.debug_tuple("HeaderError::Forbidden").finish(),
                        HeaderError::Immutable => f.debug_tuple("HeaderError::Immutable").finish(),
                    }
                }
            }
            impl core::fmt::Display for HeaderError {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    write!(f, "{:?}", self)
                }
            }
            impl core::error::Error for HeaderError {}
            const _: () = {
                assert!(1 == <HeaderError as wasmtime::component::ComponentType>::SIZE32);
                assert!(1 == <HeaderError as wasmtime::component::ComponentType>::ALIGN32);
            };
            /// This type enumerates the different kinds of errors that may occur when
            /// setting fields of a `request-options` resource.
            #[derive(
                wasmtime::component::ComponentType,
                wasmtime::component::Lift,
                wasmtime::component::Lower,
            )]
            #[component(variant)]
            #[derive(Clone, Copy)]
            pub enum RequestOptionsError {
                /// Indicates the specified field is not supported by this implementation.
                #[component(name = "not-supported")]
                NotSupported,
                /// Indicates that the operation on the `request-options` was not permitted
                /// because it is immutable.
                #[component(name = "immutable")]
                Immutable,
            }
            impl core::fmt::Debug for RequestOptionsError {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        RequestOptionsError::NotSupported => {
                            f.debug_tuple("RequestOptionsError::NotSupported").finish()
                        }
                        RequestOptionsError::Immutable => {
                            f.debug_tuple("RequestOptionsError::Immutable").finish()
                        }
                    }
                }
            }
            impl core::fmt::Display for RequestOptionsError {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    write!(f, "{:?}", self)
                }
            }
            impl core::error::Error for RequestOptionsError {}
            const _: () = {
                assert!(1 == <RequestOptionsError as wasmtime::component::ComponentType>::SIZE32);
                assert!(1 == <RequestOptionsError as wasmtime::component::ComponentType>::ALIGN32);
            };
            /// Field keys are always strings.
            pub type FieldKey = wasmtime::component::__internal::String;
            const _: () = {
                assert!(8 == <FieldKey as wasmtime::component::ComponentType>::SIZE32);
                assert!(4 == <FieldKey as wasmtime::component::ComponentType>::ALIGN32);
            };
            /// Field values should always be ASCII strings. However, in
            /// reality, HTTP implementations often have to interpret malformed values,
            /// so they are provided as a list of bytes.
            pub type FieldValue = wasmtime::component::__internal::Vec<u8>;
            const _: () = {
                assert!(8 == <FieldValue as wasmtime::component::ComponentType>::SIZE32);
                assert!(4 == <FieldValue as wasmtime::component::ComponentType>::ALIGN32);
            };
            /// This following block defines the `fields` resource which corresponds to
            /// HTTP standard Fields. Fields are a common representation used for both
            /// Headers and Trailers.
            ///
            /// A `fields` may be mutable or immutable. A `fields` created using the
            /// constructor, `from-list`, or `clone` will be mutable, but a `fields`
            /// resource given by other means (including, but not limited to,
            /// `request.headers`) might be be immutable. In an immutable fields, the
            /// `set`, `append`, and `delete` operations will fail with
            /// `header-error.immutable`.
            pub use super::super::super::__with_name1 as Fields;
            #[wasmtime::component::__internal::trait_variant_make(::core::marker::Send)]
            pub trait HostFields: Sized {
                /// Construct an empty HTTP Fields.
                ///
                /// The resulting `fields` is mutable.
                fn new(&mut self) -> wasmtime::Result<wasmtime::component::Resource<Fields>>;
                /// Construct an HTTP Fields.
                ///
                /// The resulting `fields` is mutable.
                ///
                /// The list represents each key-value pair in the Fields. Keys
                /// which have multiple values are represented by multiple entries in this
                /// list with the same key.
                ///
                /// The tuple is a pair of the field key, represented as a string, and
                /// Value, represented as a list of bytes. In a valid Fields, all keys
                /// and values are valid UTF-8 strings. However, values are not always
                /// well-formed, so they are represented as a raw list of bytes.
                ///
                /// An error result will be returned if any header or value was
                /// syntactically invalid, or if a header was forbidden.
                fn from_list(
                    &mut self,
                    entries: wasmtime::component::__internal::Vec<(FieldKey, FieldValue)>,
                ) -> wasmtime::Result<Result<wasmtime::component::Resource<Fields>, HeaderError>>;
                /// Get all of the values corresponding to a key. If the key is not present
                /// in this `fields`, an empty list is returned. However, if the key is
                /// present but empty, this is represented by a list with one or more
                /// empty field-values present.
                fn get(
                    &mut self,
                    self_: wasmtime::component::Resource<Fields>,
                    name: FieldKey,
                ) -> wasmtime::Result<wasmtime::component::__internal::Vec<FieldValue>>;
                /// Returns `true` when the key is present in this `fields`. If the key is
                /// syntactically invalid, `false` is returned.
                fn has(
                    &mut self,
                    self_: wasmtime::component::Resource<Fields>,
                    name: FieldKey,
                ) -> wasmtime::Result<bool>;
                /// Set all of the values for a key. Clears any existing values for that
                /// key, if they have been set.
                ///
                /// Fails with `header-error.immutable` if the `fields` are immutable.
                fn set(
                    &mut self,
                    self_: wasmtime::component::Resource<Fields>,
                    name: FieldKey,
                    value: wasmtime::component::__internal::Vec<FieldValue>,
                ) -> wasmtime::Result<Result<(), HeaderError>>;
                /// Delete all values for a key. Does nothing if no values for the key
                /// exist.
                ///
                /// Returns any values previously corresponding to the key.
                ///
                /// Fails with `header-error.immutable` if the `fields` are immutable.
                fn delete(
                    &mut self,
                    self_: wasmtime::component::Resource<Fields>,
                    name: FieldKey,
                ) -> wasmtime::Result<
                    Result<wasmtime::component::__internal::Vec<FieldValue>, HeaderError>,
                >;
                /// Append a value for a key. Does not change or delete any existing
                /// values for that key.
                ///
                /// Fails with `header-error.immutable` if the `fields` are immutable.
                fn append(
                    &mut self,
                    self_: wasmtime::component::Resource<Fields>,
                    name: FieldKey,
                    value: FieldValue,
                ) -> wasmtime::Result<Result<(), HeaderError>>;
                /// Retrieve the full set of keys and values in the Fields. Like the
                /// constructor, the list represents each key-value pair.
                ///
                /// The outer list represents each key-value pair in the Fields. Keys
                /// which have multiple values are represented by multiple entries in this
                /// list with the same key.
                fn entries(
                    &mut self,
                    self_: wasmtime::component::Resource<Fields>,
                ) -> wasmtime::Result<wasmtime::component::__internal::Vec<(FieldKey, FieldValue)>>;
                /// Make a deep copy of the Fields. Equivelant in behavior to calling the
                /// `fields` constructor on the return value of `entries`. The resulting
                /// `fields` is mutable.
                fn clone(
                    &mut self,
                    self_: wasmtime::component::Resource<Fields>,
                ) -> wasmtime::Result<wasmtime::component::Resource<Fields>>;
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<Fields>,
                ) -> wasmtime::Result<()>;
            }
            /// Headers is an alias for Fields.
            pub type Headers = Fields;
            /// Trailers is an alias for Fields.
            pub type Trailers = Fields;
            /// Represents an HTTP Request or Response's Body.
            ///
            /// A body has both its contents - a stream of bytes - and a (possibly empty)
            /// set of trailers, indicating that the full contents of the body have been
            /// received. This resource represents the contents as a `stream<u8>` and the
            /// delivery of trailers as a `trailers`, and ensures that the user of this
            /// interface may only be consuming either the body contents or waiting on
            /// trailers at any given time.
            pub use super::super::super::__with_name0 as Body;
            #[wasmtime::component::__internal::trait_variant_make(::core::marker::Send)]
            pub trait HostBody: Sized {
                /// Construct a new `body` with the specified stream and trailers.
                fn new(
                    &mut self,
                    stream: wasmtime::component::StreamReader<u8>,
                    trailers: Option<
                        wasmtime::component::FutureReader<wasmtime::component::Resource<Trailers>>,
                    >,
                ) -> wasmtime::Result<wasmtime::component::Resource<Body>>;
                /// Returns the contents of the body, as a stream of bytes.
                ///
                /// This function may be called multiple times as long as any `stream`s
                /// returned by previous calls have been dropped first.
                fn stream(
                    &mut self,
                    self_: wasmtime::component::Resource<Body>,
                ) -> wasmtime::Result<Result<wasmtime::component::StreamReader<u8>, ()>>;
                /// Takes ownership of `body`, and returns a `trailers`.  This function will
                /// trap if a `stream` child is still alive.
                fn finish(
                    accessor: &mut wasmtime::component::Accessor<Self>,
                    this: wasmtime::component::Resource<Body>,
                ) -> impl ::core::future::Future<
                    Output = wasmtime::Result<
                        Result<Option<wasmtime::component::Resource<Trailers>>, ErrorCode>,
                    >,
                > + Send
                       + Sync
                where
                    Self: Sized;
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<Body>,
                ) -> wasmtime::Result<()>;
            }
            /// Represents an HTTP Request.
            pub use super::super::super::__with_name2 as Request;
            #[wasmtime::component::__internal::trait_variant_make(::core::marker::Send)]
            pub trait HostRequest: Sized {
                /// Construct a new `request` with a default `method` of `GET`, and
                /// `none` values for `path-with-query`, `scheme`, and `authority`.
                ///
                /// * `headers` is the HTTP Headers for the Response.
                /// * `body` is the contents of the body, as a stream of bytes.
                /// * `trailers` is an optional `future` which resolves to the HTTP Trailers
                ///   for the Response.
                /// * `options` is optional `request-options` to be used if the request is
                ///   sent over a network connection.
                ///
                /// It is possible to construct, or manipulate with the accessor functions
                /// below, an `request` with an invalid combination of `scheme`
                /// and `authority`, or `headers` which are not permitted to be sent.
                /// It is the obligation of the `handler.handle` implementation
                /// to reject invalid constructions of `request`.
                fn new(
                    &mut self,
                    headers: wasmtime::component::Resource<Headers>,
                    body: wasmtime::component::Resource<Body>,
                    options: Option<wasmtime::component::Resource<RequestOptions>>,
                ) -> wasmtime::Result<wasmtime::component::Resource<Request>>;
                /// Get the Method for the Request.
                fn method(
                    &mut self,
                    self_: wasmtime::component::Resource<Request>,
                ) -> wasmtime::Result<Method>;
                /// Set the Method for the Request. Fails if the string present in a
                /// `method.other` argument is not a syntactically valid method.
                fn set_method(
                    &mut self,
                    self_: wasmtime::component::Resource<Request>,
                    method: Method,
                ) -> wasmtime::Result<Result<(), ()>>;
                /// Get the combination of the HTTP Path and Query for the Request.  When
                /// `none`, this represents an empty Path and empty Query.
                fn path_with_query(
                    &mut self,
                    self_: wasmtime::component::Resource<Request>,
                ) -> wasmtime::Result<Option<wasmtime::component::__internal::String>>;
                /// Set the combination of the HTTP Path and Query for the Request.  When
                /// `none`, this represents an empty Path and empty Query. Fails is the
                /// string given is not a syntactically valid path and query uri component.
                fn set_path_with_query(
                    &mut self,
                    self_: wasmtime::component::Resource<Request>,
                    path_with_query: Option<wasmtime::component::__internal::String>,
                ) -> wasmtime::Result<Result<(), ()>>;
                /// Get the HTTP Related Scheme for the Request. When `none`, the
                /// implementation may choose an appropriate default scheme.
                fn scheme(
                    &mut self,
                    self_: wasmtime::component::Resource<Request>,
                ) -> wasmtime::Result<Option<Scheme>>;
                /// Set the HTTP Related Scheme for the Request. When `none`, the
                /// implementation may choose an appropriate default scheme. Fails if the
                /// string given is not a syntactically valid uri scheme.
                fn set_scheme(
                    &mut self,
                    self_: wasmtime::component::Resource<Request>,
                    scheme: Option<Scheme>,
                ) -> wasmtime::Result<Result<(), ()>>;
                /// Get the HTTP Authority for the Request. A value of `none` may be used
                /// with Related Schemes which do not require an Authority. The HTTP and
                /// HTTPS schemes always require an authority.
                fn authority(
                    &mut self,
                    self_: wasmtime::component::Resource<Request>,
                ) -> wasmtime::Result<Option<wasmtime::component::__internal::String>>;
                /// Set the HTTP Authority for the Request. A value of `none` may be used
                /// with Related Schemes which do not require an Authority. The HTTP and
                /// HTTPS schemes always require an authority. Fails if the string given is
                /// not a syntactically valid uri authority.
                fn set_authority(
                    &mut self,
                    self_: wasmtime::component::Resource<Request>,
                    authority: Option<wasmtime::component::__internal::String>,
                ) -> wasmtime::Result<Result<(), ()>>;
                /// Get the `request-options` to be associated with this request
                ///
                /// The returned `request-options` resource is immutable: `set-*` operations
                /// will fail if invoked.
                ///
                /// This `request-options` resource is a child: it must be dropped before
                /// the parent `request` is dropped, or its ownership is transfered to
                /// another component by e.g. `handler.handle`.
                fn options(
                    &mut self,
                    self_: wasmtime::component::Resource<Request>,
                ) -> wasmtime::Result<Option<wasmtime::component::Resource<RequestOptions>>>;
                /// Get the headers associated with the Request.
                ///
                /// The returned `headers` resource is immutable: `set`, `append`, and
                /// `delete` operations will fail with `header-error.immutable`.
                ///
                /// This headers resource is a child: it must be dropped before the parent
                /// `request` is dropped, or its ownership is transfered to another
                /// component by e.g. `handler.handle`.
                fn headers(
                    &mut self,
                    self_: wasmtime::component::Resource<Request>,
                ) -> wasmtime::Result<wasmtime::component::Resource<Headers>>;
                /// Get the body associated with the Request.
                ///
                /// This body resource is a child: it must be dropped before the parent
                /// `request` is dropped, or its ownership is transfered to another
                /// component by e.g. `handler.handle`.
                fn body(
                    &mut self,
                    self_: wasmtime::component::Resource<Request>,
                ) -> wasmtime::Result<wasmtime::component::Resource<Body>>;
                /// Takes ownership of the `request` and returns the `headers` and `body`.
                fn into_parts(
                    &mut self,
                    this: wasmtime::component::Resource<Request>,
                ) -> wasmtime::Result<(
                    wasmtime::component::Resource<Headers>,
                    wasmtime::component::Resource<Body>,
                )>;
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<Request>,
                ) -> wasmtime::Result<()>;
            }
            /// Parameters for making an HTTP Request. Each of these parameters is
            /// currently an optional timeout applicable to the transport layer of the
            /// HTTP protocol.
            ///
            /// These timeouts are separate from any the user may use to bound an
            /// asynchronous call.
            pub use super::super::super::__with_name3 as RequestOptions;
            #[wasmtime::component::__internal::trait_variant_make(::core::marker::Send)]
            pub trait HostRequestOptions: Sized {
                /// Construct a default `request-options` value.
                fn new(
                    &mut self,
                ) -> wasmtime::Result<wasmtime::component::Resource<RequestOptions>>;
                /// The timeout for the initial connect to the HTTP Server.
                fn connect_timeout(
                    &mut self,
                    self_: wasmtime::component::Resource<RequestOptions>,
                ) -> wasmtime::Result<Option<Duration>>;
                /// Set the timeout for the initial connect to the HTTP Server. An error
                /// return value indicates that this timeout is not supported or that this
                /// handle is immutable.
                fn set_connect_timeout(
                    &mut self,
                    self_: wasmtime::component::Resource<RequestOptions>,
                    duration: Option<Duration>,
                ) -> wasmtime::Result<Result<(), RequestOptionsError>>;
                /// The timeout for receiving the first byte of the Response body.
                fn first_byte_timeout(
                    &mut self,
                    self_: wasmtime::component::Resource<RequestOptions>,
                ) -> wasmtime::Result<Option<Duration>>;
                /// Set the timeout for receiving the first byte of the Response body. An
                /// error return value indicates that this timeout is not supported or that
                /// this handle is immutable.
                fn set_first_byte_timeout(
                    &mut self,
                    self_: wasmtime::component::Resource<RequestOptions>,
                    duration: Option<Duration>,
                ) -> wasmtime::Result<Result<(), RequestOptionsError>>;
                /// The timeout for receiving subsequent chunks of bytes in the Response
                /// body stream.
                fn between_bytes_timeout(
                    &mut self,
                    self_: wasmtime::component::Resource<RequestOptions>,
                ) -> wasmtime::Result<Option<Duration>>;
                /// Set the timeout for receiving subsequent chunks of bytes in the Response
                /// body stream. An error return value indicates that this timeout is not
                /// supported or that this handle is immutable.
                fn set_between_bytes_timeout(
                    &mut self,
                    self_: wasmtime::component::Resource<RequestOptions>,
                    duration: Option<Duration>,
                ) -> wasmtime::Result<Result<(), RequestOptionsError>>;
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<RequestOptions>,
                ) -> wasmtime::Result<()>;
            }
            /// This type corresponds to the HTTP standard Status Code.
            pub type StatusCode = u16;
            const _: () = {
                assert!(2 == <StatusCode as wasmtime::component::ComponentType>::SIZE32);
                assert!(2 == <StatusCode as wasmtime::component::ComponentType>::ALIGN32);
            };
            /// Represents an HTTP Response.
            pub use super::super::super::__with_name4 as Response;
            #[wasmtime::component::__internal::trait_variant_make(::core::marker::Send)]
            pub trait HostResponse: Sized {
                /// Construct an `response`, with a default `status-code` of `200`.  If a
                /// different `status-code` is needed, it must be set via the
                /// `set-status-code` method.
                ///
                /// * `headers` is the HTTP Headers for the Response.
                /// * `body` is the contents of the body, as a stream of bytes.
                /// * `trailers` is an optional `future` which resolves to the HTTP Trailers
                ///   for the Response.
                fn new(
                    &mut self,
                    headers: wasmtime::component::Resource<Headers>,
                    body: wasmtime::component::Resource<Body>,
                ) -> wasmtime::Result<wasmtime::component::Resource<Response>>;
                /// Get the HTTP Status Code for the Response.
                fn status_code(
                    &mut self,
                    self_: wasmtime::component::Resource<Response>,
                ) -> wasmtime::Result<StatusCode>;
                /// Set the HTTP Status Code for the Response. Fails if the status-code
                /// given is not a valid http status code.
                fn set_status_code(
                    &mut self,
                    self_: wasmtime::component::Resource<Response>,
                    status_code: StatusCode,
                ) -> wasmtime::Result<Result<(), ()>>;
                /// Get the headers associated with the Request.
                ///
                /// The returned `headers` resource is immutable: `set`, `append`, and
                /// `delete` operations will fail with `header-error.immutable`.
                ///
                /// This headers resource is a child: it must be dropped before the parent
                /// `response` is dropped, or its ownership is transfered to another
                /// component by e.g. `handler.handle`.
                fn headers(
                    &mut self,
                    self_: wasmtime::component::Resource<Response>,
                ) -> wasmtime::Result<wasmtime::component::Resource<Headers>>;
                /// Get the body associated with the Response.
                ///
                /// This body resource is a child: it must be dropped before the parent
                /// `response` is dropped, or its ownership is transfered to another
                /// component by e.g. `handler.handle`.
                fn body(
                    &mut self,
                    self_: wasmtime::component::Resource<Response>,
                ) -> wasmtime::Result<wasmtime::component::Resource<Body>>;
                /// Takes ownership of the `response` and returns the `headers` and `body`.
                fn into_parts(
                    &mut self,
                    this: wasmtime::component::Resource<Response>,
                ) -> wasmtime::Result<(
                    wasmtime::component::Resource<Headers>,
                    wasmtime::component::Resource<Body>,
                )>;
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<Response>,
                ) -> wasmtime::Result<()>;
            }
            #[wasmtime::component::__internal::trait_variant_make(::core::marker::Send)]
            pub trait Host:
                Send
                + HostFields
                + HostBody
                + HostRequest
                + HostRequestOptions
                + HostResponse
                + Sized
            {
                /// Attempts to extract a http-related `error-code` from the stream `error`
                /// provided.
                ///
                /// Stream operations may fail with a stream `error` with more information
                /// about the operation that failed. This `error` can be passed to this
                /// function to see if there's http-related information about the error to
                /// return.
                ///
                /// Note that this function is fallible because not all stream errors are
                /// http-related errors.
                fn http_error_code(
                    &mut self,
                    err: wasmtime::component::ErrorContext,
                ) -> wasmtime::Result<Option<ErrorCode>>;
            }

            pub trait GetHost<T>:
                Fn(T) -> <Self as GetHost<T>>::Host + Send + Sync + Copy + 'static
            {
                type Host: Host + Send;
            }

            impl<F, T, O> GetHost<T> for F
            where
                F: Fn(T) -> O + Send + Sync + Copy + 'static,
                O: Host + Send,
            {
                type Host = O;
            }

            pub fn add_to_linker_get_host<T, G: for<'a> GetHost<&'a mut T>>(
                linker: &mut wasmtime::component::Linker<T>,
                host_getter: G,
            ) -> wasmtime::Result<()>
            where
                T: Send + 'static,
            {
                let mut inst = linker.instance("wasi:http/types@0.3.0-draft")?;
                inst.resource(
                    "fields",
                    wasmtime::component::ResourceType::host::<Fields>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        HostFields::drop(
                            &mut host_getter(store.data_mut()),
                            wasmtime::component::Resource::new_own(rep),
                        )
                    },
                )?;
                inst.resource(
                    "body",
                    wasmtime::component::ResourceType::host::<Body>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        HostBody::drop(
                            &mut host_getter(store.data_mut()),
                            wasmtime::component::Resource::new_own(rep),
                        )
                    },
                )?;
                inst.resource(
                    "request",
                    wasmtime::component::ResourceType::host::<Request>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        HostRequest::drop(
                            &mut host_getter(store.data_mut()),
                            wasmtime::component::Resource::new_own(rep),
                        )
                    },
                )?;
                inst.resource(
                    "request-options",
                    wasmtime::component::ResourceType::host::<RequestOptions>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        HostRequestOptions::drop(
                            &mut host_getter(store.data_mut()),
                            wasmtime::component::Resource::new_own(rep),
                        )
                    },
                )?;
                inst.resource(
                    "response",
                    wasmtime::component::ResourceType::host::<Response>(),
                    move |mut store, rep| -> wasmtime::Result<()> {
                        HostResponse::drop(
                            &mut host_getter(store.data_mut()),
                            wasmtime::component::Resource::new_own(rep),
                        )
                    },
                )?;
                inst.func_wrap(
                    "http-error-code",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0,): (wasmtime::component::ErrorContext,)| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = Host::http_error_code(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[constructor]fields",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = HostFields::new(host);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[static]fields.from-list",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0,): (
                        wasmtime::component::__internal::Vec<(FieldKey, FieldValue)>,
                    )| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = HostFields::from_list(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap("[method]fields.get", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (wasmtime::component::Resource<Fields>, FieldKey, )| {
                    let host = &mut host_getter(caller.data_mut());
                    let r = HostFields::get(host, arg0,arg1,);
                    Ok((r?,))
                  }
                  )?;
                inst.func_wrap("[method]fields.has", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (wasmtime::component::Resource<Fields>, FieldKey, )| {
                    let host = &mut host_getter(caller.data_mut());
                    let r = HostFields::has(host, arg0,arg1,);
                    Ok((r?,))
                  }
                  )?;
                inst.func_wrap(
                    "[method]fields.set",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1, arg2): (
                        wasmtime::component::Resource<Fields>,
                        FieldKey,
                        wasmtime::component::__internal::Vec<FieldValue>,
                    )| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = HostFields::set(host, arg0, arg1, arg2);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap("[method]fields.delete", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (wasmtime::component::Resource<Fields>, FieldKey, )| {
                    let host = &mut host_getter(caller.data_mut());
                    let r = HostFields::delete(host, arg0,arg1,);
                    Ok((r?,))
                  }
                  )?;
                inst.func_wrap(
                    "[method]fields.append",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1, arg2): (
                        wasmtime::component::Resource<Fields>,
                        FieldKey,
                        FieldValue,
                    )| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = HostFields::append(host, arg0, arg1, arg2);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]fields.entries",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0,): (wasmtime::component::Resource<Fields>,)| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = HostFields::entries(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]fields.clone",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0,): (wasmtime::component::Resource<Fields>,)| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = HostFields::clone(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[constructor]body",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1): (
                        wasmtime::component::StreamReader<u8>,
                        Option<
                            wasmtime::component::FutureReader<
                                wasmtime::component::Resource<Trailers>,
                            >,
                        >,
                    )| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = HostBody::new(host, arg0, arg1);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]body.stream",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0,): (wasmtime::component::Resource<Body>,)| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = HostBody::stream(host, arg0);
                        Ok((r?,))
                    },
                )?;

                thread_local! {
                    static FINISH_HOST: std::cell::Cell<*mut u8> = std::cell::Cell::new(std::ptr::null_mut());
                    static FINISH_SPAWNED: std::cell::RefCell<Vec<std::pin::Pin<Box<dyn std::future::Future<Output = wasmtime::Result<()>> + Send + Sync + 'static>>>> = std::cell::RefCell::new(Vec::new());
                }

                fn poll<T, G: for<'a> GetHost<&'a mut T>, F: std::future::Future + ?Sized>(
                    getter: G,
                    store: wasmtime::VMStoreRawPtr,
                    cx: &mut std::task::Context,
                    future: std::pin::Pin<&mut F>,
                ) -> std::task::Poll<F::Output> {
                    let mut store_cx =
                        unsafe { wasmtime::StoreContextMut::new(&mut *store.0.as_ptr().cast()) };
                    let result = {
                        let host = &mut getter(store_cx.data_mut());
                        let old = FINISH_HOST.with(|v| v.replace((host as *mut G::Host).cast()));
                        let result = future.poll(cx);
                        // TODO: use RAII to reset this:
                        FINISH_HOST.with(|v| v.set(old));
                        result
                    };
                    for mut future in FINISH_SPAWNED.with(|v| {
                        std::mem::take(std::ops::DerefMut::deref_mut(&mut v.borrow_mut()))
                    }) {
                        store_cx.spawn(futures::future::poll_fn(move |cx| {
                            poll(getter, store, cx, future.as_mut())
                        }))
                    }
                    result
                }

                inst.func_wrap_concurrent(
                    "[static]body.finish",
                    move |caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0,): (wasmtime::component::Resource<Body>,)| {
                        let mut accessor = unsafe {
                            wasmtime::component::Accessor::<G::Host>::new(
                                || FINISH_HOST.with(|v| v.get()).cast(),
                                |future| FINISH_SPAWNED.with(|v| v.borrow_mut().push(future)),
                            )
                        };
                        let mut future = wasmtime::component::__internal::Box::pin(async move {
                            let r = <G::Host as HostBody>::finish(&mut accessor, arg0).await;
                            Ok((r?,))
                        });
                        let store = wasmtime::VMStoreRawPtr(caller.traitobj());
                        wasmtime::component::__internal::Box::pin(futures::future::poll_fn(
                            move |cx| poll(host_getter, store, cx, future.as_mut()),
                        ))
                    },
                )?;

                inst.func_wrap(
                    "[constructor]request",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1, arg2): (
                        wasmtime::component::Resource<Headers>,
                        wasmtime::component::Resource<Body>,
                        Option<wasmtime::component::Resource<RequestOptions>>,
                    )| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = HostRequest::new(host, arg0, arg1, arg2);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]request.method",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0,): (wasmtime::component::Resource<Request>,)| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = HostRequest::method(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap("[method]request.set-method", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (wasmtime::component::Resource<Request>, Method, )| {
                    let host = &mut host_getter(caller.data_mut());
                    let r = HostRequest::set_method(host, arg0,arg1,);
                    Ok((r?,))
                  }
                  )?;
                inst.func_wrap(
                    "[method]request.path-with-query",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0,): (wasmtime::component::Resource<Request>,)| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = HostRequest::path_with_query(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]request.set-path-with-query",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1): (
                        wasmtime::component::Resource<Request>,
                        Option<wasmtime::component::__internal::String>,
                    )| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = HostRequest::set_path_with_query(host, arg0, arg1);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]request.scheme",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0,): (wasmtime::component::Resource<Request>,)| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = HostRequest::scheme(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]request.set-scheme",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1): (
                        wasmtime::component::Resource<Request>,
                        Option<Scheme>,
                    )| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = HostRequest::set_scheme(host, arg0, arg1);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]request.authority",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0,): (wasmtime::component::Resource<Request>,)| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = HostRequest::authority(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]request.set-authority",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1): (
                        wasmtime::component::Resource<Request>,
                        Option<wasmtime::component::__internal::String>,
                    )| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = HostRequest::set_authority(host, arg0, arg1);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]request.options",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0,): (wasmtime::component::Resource<Request>,)| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = HostRequest::options(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]request.headers",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0,): (wasmtime::component::Resource<Request>,)| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = HostRequest::headers(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]request.body",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0,): (wasmtime::component::Resource<Request>,)| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = HostRequest::body(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[static]request.into-parts",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0,): (wasmtime::component::Resource<Request>,)| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = HostRequest::into_parts(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[constructor]request-options",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>, (): ()| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = HostRequestOptions::new(host);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap("[method]request-options.connect-timeout", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (wasmtime::component::Resource<RequestOptions>, )| {
                    let host = &mut host_getter(caller.data_mut());
                    let r = HostRequestOptions::connect_timeout(host, arg0,);
                    Ok((r?,))
                  }
                  )?;
                inst.func_wrap(
                    "[method]request-options.set-connect-timeout",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1): (
                        wasmtime::component::Resource<RequestOptions>,
                        Option<Duration>,
                    )| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = HostRequestOptions::set_connect_timeout(host, arg0, arg1);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap("[method]request-options.first-byte-timeout", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (wasmtime::component::Resource<RequestOptions>, )| {
                    let host = &mut host_getter(caller.data_mut());
                    let r = HostRequestOptions::first_byte_timeout(host, arg0,);
                    Ok((r?,))
                  }
                  )?;
                inst.func_wrap(
                    "[method]request-options.set-first-byte-timeout",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1): (
                        wasmtime::component::Resource<RequestOptions>,
                        Option<Duration>,
                    )| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = HostRequestOptions::set_first_byte_timeout(host, arg0, arg1);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap("[method]request-options.between-bytes-timeout", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,) : (wasmtime::component::Resource<RequestOptions>, )| {
                    let host = &mut host_getter(caller.data_mut());
                    let r = HostRequestOptions::between_bytes_timeout(host, arg0,);
                    Ok((r?,))
                  }
                  )?;
                inst.func_wrap(
                    "[method]request-options.set-between-bytes-timeout",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1): (
                        wasmtime::component::Resource<RequestOptions>,
                        Option<Duration>,
                    )| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = HostRequestOptions::set_between_bytes_timeout(host, arg0, arg1);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[constructor]response",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0, arg1): (
                        wasmtime::component::Resource<Headers>,
                        wasmtime::component::Resource<Body>,
                    )| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = HostResponse::new(host, arg0, arg1);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]response.status-code",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0,): (wasmtime::component::Resource<Response>,)| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = HostResponse::status_code(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap("[method]response.set-status-code", move |mut caller: wasmtime::StoreContextMut<'_, T>, (arg0,arg1,) : (wasmtime::component::Resource<Response>, StatusCode, )| {
                    let host = &mut host_getter(caller.data_mut());
                    let r = HostResponse::set_status_code(host, arg0,arg1,);
                    Ok((r?,))
                  }
                  )?;
                inst.func_wrap(
                    "[method]response.headers",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0,): (wasmtime::component::Resource<Response>,)| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = HostResponse::headers(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[method]response.body",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0,): (wasmtime::component::Resource<Response>,)| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = HostResponse::body(host, arg0);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "[static]response.into-parts",
                    move |mut caller: wasmtime::StoreContextMut<'_, T>,
                          (arg0,): (wasmtime::component::Resource<Response>,)| {
                        let host = &mut host_getter(caller.data_mut());
                        let r = HostResponse::into_parts(host, arg0);
                        Ok((r?,))
                    },
                )?;
                Ok(())
            }
        }

        #[allow(clippy::all)]
        pub mod handler {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::{anyhow, Box};

            pub type Request = super::super::super::wasi::http::types::Request;
            pub type Response = super::super::super::wasi::http::types::Response;
            pub type ErrorCode = super::super::super::wasi::http::types::ErrorCode;
            const _: () = {
                assert!(32 == <ErrorCode as wasmtime::component::ComponentType>::SIZE32);
                assert!(8 == <ErrorCode as wasmtime::component::ComponentType>::ALIGN32);
            };
            #[wasmtime::component::__internal::trait_variant_make(::core::marker::Send)]
            pub trait Host: Send {
                /// When exported, this function may be called with either an incoming
                /// request read from the network or a request synthesized or forwarded by
                /// another component.
                ///
                /// When imported, this function may be used to either send an outgoing
                /// request over the network or pass it to another component.
                fn handle(
                    accessor: &mut wasmtime::component::Accessor<Self>,
                    request: wasmtime::component::Resource<Request>,
                ) -> impl ::core::future::Future<
                    Output = wasmtime::Result<
                        Result<wasmtime::component::Resource<Response>, ErrorCode>,
                    >,
                > + Send
                       + Sync
                where
                    Self: Sized;
            }

            pub trait GetHost<T, D>:
                Fn(T) -> <Self as GetHost<T, D>>::Host + Send + Sync + Copy + 'static
            {
                type Host: Host + Send;
            }

            impl<F, T, D, O> GetHost<T, D> for F
            where
                F: Fn(T) -> O + Send + Sync + Copy + 'static,
                O: Host + Send,
            {
                type Host = O;
            }

            // pub fn add_to_linker_get_host<T, G: for<'a> GetHost<&'a mut T, T, Host: Host + Send>>(
            //     linker: &mut wasmtime::component::Linker<T>,

            //     host_getter: G,
            // ) -> wasmtime::Result<()>
            // where
            //     T: Send + 'static,
            // {
            //     let mut inst = linker.instance("wasi:http/handler@0.3.0-draft")?;
            //     inst.func_wrap_concurrent(
            //         "handle",
            //         move |caller: wasmtime::StoreContextMut<'_, T>,
            //               (arg0,): (wasmtime::component::Resource<Request>,)| {
            //             _ = host_getter;
            //             let mut accessor = unsafe {
            //                 wasmtime::component::Accessor::<T>::new(caller.traitobj().as_ptr())
            //             };
            //             wasmtime::component::__internal::Box::pin(async move {
            //                 let r = <G::Host as Host>::handle(&mut accessor, arg0).await;
            //                 Ok((r?,))
            //             })
            //         },
            //     )?;
            //     Ok(())
            // }
        }
    }
}

// wasmtime::component::bindgen!({
//     trappable_imports: true,
//     path: "wit",
//     interfaces: "
//       import wasi:http/types@0.3.0-draft;
//       import wasi:http/handler@0.3.0-draft;
//     ",
//     concurrent_imports: true,
//     async: {
//         only_imports: [
//             "wasi:http/types@0.3.0-draft#[static]body.finish",
//             "wasi:http/handler@0.3.0-draft#handle",
//         ]
//     },
//     with: {
//         "wasi:http/types/body": Body,
//         "wasi:http/types/request": Request,
//         "wasi:http/types/request-options": RequestOptions,
//         "wasi:http/types/response": Response,
//         "wasi:http/types/fields": Fields,
//     }, debug: true
// });

use {
    anyhow::anyhow,
    std::{fmt, future::Future, mem},
    wasi::http::types::{ErrorCode, HeaderError, Method, RequestOptionsError, Scheme},
    wasmtime::component::{
        Accessor, ErrorContext, FutureReader, Resource, ResourceTable, StreamReader,
    },
};

impl fmt::Display for Scheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Scheme::Http => "http",
                Scheme::Https => "https",
                Scheme::Other(s) => s,
            }
        )
    }
}

pub trait WasiHttpView: Send + Sized {
    fn table(&mut self) -> &mut ResourceTable;

    fn send_request(
        accessor: &mut Accessor<Self>,
        request: Resource<Request>,
    ) -> impl Future<Output = wasmtime::Result<Result<Resource<Response>, ErrorCode>>> + Send + Sync;
}

pub struct Body {
    pub stream: Option<StreamReader<u8>>,
    pub trailers: Option<FutureReader<Resource<Fields>>>,
}

#[derive(Clone)]
pub struct Fields(pub Vec<(String, Vec<u8>)>);

#[derive(Default, Copy, Clone)]
pub struct RequestOptions {
    pub connect_timeout: Option<u64>,
    pub first_byte_timeout: Option<u64>,
    pub between_bytes_timeout: Option<u64>,
}

pub struct Request {
    pub method: Method,
    pub scheme: Option<Scheme>,
    pub path_with_query: Option<String>,
    pub authority: Option<String>,
    pub headers: Fields,
    pub body: Body,
    pub options: Option<RequestOptions>,
}

pub struct Response {
    pub status_code: u16,
    pub headers: Fields,
    pub body: Body,
}

impl<T: WasiHttpView> wasi::http::types::HostFields for T {
    fn new(&mut self) -> wasmtime::Result<Resource<Fields>> {
        Ok(self.table().push(Fields(Vec::new()))?)
    }

    fn from_list(
        &mut self,
        list: Vec<(String, Vec<u8>)>,
    ) -> wasmtime::Result<Result<Resource<Fields>, HeaderError>> {
        Ok(Ok(self.table().push(Fields(list))?))
    }

    fn get(&mut self, this: Resource<Fields>, key: String) -> wasmtime::Result<Vec<Vec<u8>>> {
        Ok(self
            .table()
            .get(&this)?
            .0
            .iter()
            .filter(|(k, _)| *k == key)
            .map(|(_, v)| v.clone())
            .collect())
    }

    fn has(&mut self, this: Resource<Fields>, key: String) -> wasmtime::Result<bool> {
        Ok(self.table().get(&this)?.0.iter().any(|(k, _)| *k == key))
    }

    fn set(
        &mut self,
        this: Resource<Fields>,
        key: String,
        values: Vec<Vec<u8>>,
    ) -> wasmtime::Result<Result<(), HeaderError>> {
        let fields = self.table().get_mut(&this)?;
        fields.0.retain(|(k, _)| *k != key);
        fields
            .0
            .extend(values.into_iter().map(|v| (key.clone(), v)));
        Ok(Ok(()))
    }

    fn delete(
        &mut self,
        this: Resource<Fields>,
        key: String,
    ) -> wasmtime::Result<Result<Vec<Vec<u8>>, HeaderError>> {
        let fields = self.table().get_mut(&this)?;
        let (matched, unmatched) = mem::take(&mut fields.0)
            .into_iter()
            .partition(|(k, _)| *k == key);
        fields.0 = unmatched;
        Ok(Ok(matched.into_iter().map(|(_, v)| v).collect()))
    }

    fn append(
        &mut self,
        this: Resource<Fields>,
        key: String,
        value: Vec<u8>,
    ) -> wasmtime::Result<Result<(), HeaderError>> {
        self.table().get_mut(&this)?.0.push((key, value));
        Ok(Ok(()))
    }

    fn entries(&mut self, this: Resource<Fields>) -> wasmtime::Result<Vec<(String, Vec<u8>)>> {
        Ok(self.table().get(&this)?.0.clone())
    }

    fn clone(&mut self, this: Resource<Fields>) -> wasmtime::Result<Resource<Fields>> {
        let entries = self.table().get(&this)?.0.clone();
        Ok(self.table().push(Fields(entries))?)
    }

    fn drop(&mut self, this: Resource<Fields>) -> wasmtime::Result<()> {
        self.table().delete(this)?;
        Ok(())
    }
}

impl<T: WasiHttpView> wasi::http::types::HostBody for T {
    fn new(
        &mut self,
        stream: StreamReader<u8>,
        trailers: Option<FutureReader<Resource<Fields>>>,
    ) -> wasmtime::Result<Resource<Body>> {
        Ok(self.table().push(Body {
            stream: Some(stream),
            trailers,
        })?)
    }

    fn stream(&mut self, this: Resource<Body>) -> wasmtime::Result<Result<StreamReader<u8>, ()>> {
        // TODO: This should return a child handle
        let stream = self.table().get_mut(&this)?.stream.take().ok_or_else(|| {
            anyhow!("todo: allow wasi:http/types#body.stream to be called multiple times")
        })?;

        Ok(Ok(stream))
    }

    async fn finish(
        accessor: &mut Accessor<Self>,
        this: Resource<Body>,
    ) -> wasmtime::Result<Result<Option<Resource<Fields>>, ErrorCode>> {
        let _trailers =
            accessor.with(|me| Ok::<_, anyhow::Error>(me.table().delete(this)?.trailers))?;

        todo!()
    }

    fn drop(&mut self, this: Resource<Body>) -> wasmtime::Result<()> {
        self.table().delete(this)?;
        Ok(())
    }
}

impl<T: WasiHttpView> wasi::http::types::HostRequest for T {
    fn new(
        &mut self,
        headers: Resource<Fields>,
        body: Resource<Body>,
        options: Option<Resource<RequestOptions>>,
    ) -> wasmtime::Result<Resource<Request>> {
        let headers = self.table().delete(headers)?;
        let body = self.table().delete(body)?;
        let options = if let Some(options) = options {
            Some(self.table().delete(options)?)
        } else {
            None
        };

        Ok(self.table().push(Request {
            method: Method::Get,
            scheme: None,
            path_with_query: None,
            authority: None,
            headers,
            body,
            options,
        })?)
    }

    fn method(&mut self, this: Resource<Request>) -> wasmtime::Result<Method> {
        Ok(self.table().get(&this)?.method.clone())
    }

    fn set_method(
        &mut self,
        this: Resource<Request>,
        method: Method,
    ) -> wasmtime::Result<Result<(), ()>> {
        self.table().get_mut(&this)?.method = method;
        Ok(Ok(()))
    }

    fn scheme(&mut self, this: Resource<Request>) -> wasmtime::Result<Option<Scheme>> {
        Ok(self.table().get(&this)?.scheme.clone())
    }

    fn set_scheme(
        &mut self,
        this: Resource<Request>,
        scheme: Option<Scheme>,
    ) -> wasmtime::Result<Result<(), ()>> {
        self.table().get_mut(&this)?.scheme = scheme;
        Ok(Ok(()))
    }

    fn path_with_query(&mut self, this: Resource<Request>) -> wasmtime::Result<Option<String>> {
        Ok(self.table().get(&this)?.path_with_query.clone())
    }

    fn set_path_with_query(
        &mut self,
        this: Resource<Request>,
        path_with_query: Option<String>,
    ) -> wasmtime::Result<Result<(), ()>> {
        self.table().get_mut(&this)?.path_with_query = path_with_query;
        Ok(Ok(()))
    }

    fn authority(&mut self, this: Resource<Request>) -> wasmtime::Result<Option<String>> {
        Ok(self.table().get(&this)?.authority.clone())
    }

    fn set_authority(
        &mut self,
        this: Resource<Request>,
        authority: Option<String>,
    ) -> wasmtime::Result<Result<(), ()>> {
        self.table().get_mut(&this)?.authority = authority;
        Ok(Ok(()))
    }

    fn options(
        &mut self,
        this: Resource<Request>,
    ) -> wasmtime::Result<Option<Resource<RequestOptions>>> {
        // TODO: This should return an immutable child handle
        let options = self.table().get(&this)?.options;
        Ok(if let Some(options) = options {
            Some(self.table().push(options)?)
        } else {
            None
        })
    }

    fn headers(&mut self, this: Resource<Request>) -> wasmtime::Result<Resource<Fields>> {
        // TODO: This should return an immutable child handle
        let headers = self.table().get(&this)?.headers.clone();
        Ok(self.table().push(headers)?)
    }

    fn body(&mut self, _this: Resource<Request>) -> wasmtime::Result<Resource<Body>> {
        Err(anyhow!("todo: implement wasi:http/types#request.body"))
    }

    fn into_parts(
        &mut self,
        this: Resource<Request>,
    ) -> wasmtime::Result<(Resource<Fields>, Resource<Body>)> {
        let request = self.table().delete(this)?;
        let headers = self.table().push(request.headers)?;
        let body = self.table().push(request.body)?;
        Ok((headers, body))
    }

    fn drop(&mut self, this: Resource<Request>) -> wasmtime::Result<()> {
        self.table().delete(this)?;
        Ok(())
    }
}

impl<T: WasiHttpView> wasi::http::types::HostResponse for T {
    fn new(
        &mut self,
        headers: Resource<Fields>,
        body: Resource<Body>,
    ) -> wasmtime::Result<Resource<Response>> {
        let headers = self.table().delete(headers)?;
        let body = self.table().delete(body)?;

        Ok(self.table().push(Response {
            status_code: 200,
            headers,
            body,
        })?)
    }

    fn status_code(&mut self, this: Resource<Response>) -> wasmtime::Result<u16> {
        Ok(self.table().get(&this)?.status_code)
    }

    fn set_status_code(
        &mut self,
        this: Resource<Response>,
        status_code: u16,
    ) -> wasmtime::Result<Result<(), ()>> {
        self.table().get_mut(&this)?.status_code = status_code;
        Ok(Ok(()))
    }

    fn headers(&mut self, this: Resource<Response>) -> wasmtime::Result<Resource<Fields>> {
        // TODO: This should return an immutable child handle
        let headers = self.table().get(&this)?.headers.clone();
        Ok(self.table().push(headers)?)
    }

    fn body(&mut self, _this: Resource<Response>) -> wasmtime::Result<Resource<Body>> {
        Err(anyhow!("todo: implement wasi:http/types#response.body"))
    }

    fn into_parts(
        &mut self,
        this: Resource<Response>,
    ) -> wasmtime::Result<(Resource<Fields>, Resource<Body>)> {
        let response = self.table().delete(this)?;
        let headers = self.table().push(response.headers)?;
        let body = self.table().push(response.body)?;
        Ok((headers, body))
    }

    fn drop(&mut self, this: Resource<Response>) -> wasmtime::Result<()> {
        self.table().delete(this)?;
        Ok(())
    }
}

impl<T: WasiHttpView> wasi::http::types::HostRequestOptions for T {
    fn new(&mut self) -> wasmtime::Result<Resource<RequestOptions>> {
        Ok(self.table().push(RequestOptions::default())?)
    }

    fn connect_timeout(&mut self, this: Resource<RequestOptions>) -> wasmtime::Result<Option<u64>> {
        Ok(self.table().get(&this)?.connect_timeout)
    }

    fn set_connect_timeout(
        &mut self,
        this: Resource<RequestOptions>,
        connect_timeout: Option<u64>,
    ) -> wasmtime::Result<Result<(), RequestOptionsError>> {
        self.table().get_mut(&this)?.connect_timeout = connect_timeout;
        Ok(Ok(()))
    }

    fn first_byte_timeout(
        &mut self,
        this: Resource<RequestOptions>,
    ) -> wasmtime::Result<Option<u64>> {
        Ok(self.table().get(&this)?.first_byte_timeout)
    }

    fn set_first_byte_timeout(
        &mut self,
        this: Resource<RequestOptions>,
        first_byte_timeout: Option<u64>,
    ) -> wasmtime::Result<Result<(), RequestOptionsError>> {
        self.table().get_mut(&this)?.first_byte_timeout = first_byte_timeout;
        Ok(Ok(()))
    }

    fn between_bytes_timeout(
        &mut self,
        this: Resource<RequestOptions>,
    ) -> wasmtime::Result<Option<u64>> {
        Ok(self.table().get(&this)?.between_bytes_timeout)
    }

    fn set_between_bytes_timeout(
        &mut self,
        this: Resource<RequestOptions>,
        between_bytes_timeout: Option<u64>,
    ) -> wasmtime::Result<Result<(), RequestOptionsError>> {
        self.table().get_mut(&this)?.between_bytes_timeout = between_bytes_timeout;
        Ok(Ok(()))
    }

    fn drop(&mut self, this: Resource<RequestOptions>) -> wasmtime::Result<()> {
        self.table().delete(this)?;
        Ok(())
    }
}

impl<T: WasiHttpView> wasi::http::types::Host for T {
    fn http_error_code(&mut self, _error: ErrorContext) -> wasmtime::Result<Option<ErrorCode>> {
        Err(anyhow!("todo: implement wasi:http/types#http-error-code"))
    }
}

impl<T: WasiHttpView> wasi::http::handler::Host for T {
    async fn handle(
        accessor: &mut Accessor<Self>,
        request: Resource<Request>,
    ) -> wasmtime::Result<Result<Resource<Response>, ErrorCode>> {
        Self::send_request(accessor, request).await
    }
}
