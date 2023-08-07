/// Request for `healthCheck` and `healthWatch`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheckRequest {
    #[prost(string, tag = "1")]
    pub service: ::prost::alloc::string::String,
}
/// Response for `healthCheck` and `healthWatch`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheckResponse {
    #[prost(enumeration = "health_check_response::ServingStatus", tag = "1")]
    pub status: i32,
}
/// Nested message and enum types in `HealthCheckResponse`.
pub mod health_check_response {
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
    pub enum ServingStatus {
        Unknown = 0,
        Serving = 1,
        NotServing = 2,
        /// Used only by the Watch method.
        ServiceUnknown = 3,
    }
    impl ServingStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ServingStatus::Unknown => "UNKNOWN",
                ServingStatus::Serving => "SERVING",
                ServingStatus::NotServing => "NOT_SERVING",
                ServingStatus::ServiceUnknown => "SERVICE_UNKNOWN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "SERVING" => Some(Self::Serving),
                "NOT_SERVING" => Some(Self::NotServing),
                "SERVICE_UNKNOWN" => Some(Self::ServiceUnknown),
                _ => None,
            }
        }
    }
}
