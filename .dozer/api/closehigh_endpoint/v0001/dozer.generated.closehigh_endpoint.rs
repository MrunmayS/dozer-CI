/// Request for `count` and `query`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClosehighEndpointsRequest {
    /// JSON query string.
    #[prost(string, optional, tag = "1")]
    pub query: ::core::option::Option<::prost::alloc::string::String>,
}
/// Response for `count`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CountClosehighEndpointsResponse {
    /// The number of records.
    #[prost(uint64, tag = "1")]
    pub count: u64,
}
/// Response for `query`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClosehighEndpointsResponse {
    /// The list of records.
    #[prost(message, repeated, tag = "1")]
    pub records: ::prost::alloc::vec::Vec<ClosehighEndpointWithId>,
}
/// *
/// ClosehighEndpoint record type.
///
/// Nullable fields will be generated as `optional` fields.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClosehighEndpoint {
    #[prost(int64, optional, tag = "1")]
    pub tp: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "2")]
    pub hi: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub lo: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub wp: ::core::option::Option<::prost::alloc::string::String>,
    /// Records with same primary key will have increasing version.
    #[prost(uint32, tag = "5")]
    pub dozer_record_version: u32,
}
/// A record with its id in cache.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClosehighEndpointWithId {
    /// The record id.
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// The record data.
    #[prost(message, optional, tag = "2")]
    pub record: ::core::option::Option<ClosehighEndpoint>,
}
