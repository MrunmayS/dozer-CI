/// Request for `count` and `query`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRequest {
    /// The name of the endpoint to query.
    #[prost(string, tag = "1")]
    pub endpoint: ::prost::alloc::string::String,
    /// JSON query string.
    #[prost(string, optional, tag = "2")]
    pub query: ::core::option::Option<::prost::alloc::string::String>,
}
/// Response for `count`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CountResponse {
    /// The number of records satisfying the query.
    #[prost(uint64, tag = "1")]
    pub count: u64,
}
/// Request for `OnEvent`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnEventRequest {
    /// The event type to subscribe to.
    #[prost(enumeration = "super::types::EventType", tag = "1")]
    pub r#type: i32,
    /// The name of the endpoint to subscribe to.
    #[prost(string, tag = "2")]
    pub endpoint: ::prost::alloc::string::String,
    /// JSON filter string.
    #[prost(string, optional, tag = "3")]
    pub filter: ::core::option::Option<::prost::alloc::string::String>,
}
/// Request for `getFields`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFieldsRequest {
    /// The endpoint name.
    #[prost(string, tag = "1")]
    pub endpoint: ::prost::alloc::string::String,
}
/// Response for `getFields`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFieldsResponse {
    /// The list of indexes of the keys that are used as the primary index.
    #[prost(int32, repeated, tag = "1")]
    pub primary_index: ::prost::alloc::vec::Vec<i32>,
    /// The list of field definitions.
    #[prost(message, repeated, tag = "2")]
    pub fields: ::prost::alloc::vec::Vec<super::types::FieldDefinition>,
}
/// Response for `query`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResponse {
    /// The list of field definitions.
    #[prost(message, repeated, tag = "1")]
    pub fields: ::prost::alloc::vec::Vec<super::types::FieldDefinition>,
    /// The list of record data.
    #[prost(message, repeated, tag = "2")]
    pub records: ::prost::alloc::vec::Vec<super::types::RecordWithId>,
}
/// Request for `getEndpoints`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEndpointsRequest {}
/// Response for `getEndpoints`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEndpointsResponse {
    /// List of endpoint names.
    #[prost(string, repeated, tag = "1")]
    pub endpoints: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
