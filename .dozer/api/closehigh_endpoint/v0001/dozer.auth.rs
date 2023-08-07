/// Request for `GetAuthTokenRequest`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAuthTokenRequest {
    #[prost(string, tag = "1")]
    pub access_filter: ::prost::alloc::string::String,
}
/// Response for `GetAuthTokenResponse`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAuthTokenResponse {
    /// Generate token for access
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
}
