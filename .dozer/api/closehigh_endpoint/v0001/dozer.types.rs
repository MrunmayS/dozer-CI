/// A Dozer event.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operation {
    /// The operation type.
    #[prost(enumeration = "OperationType", tag = "1")]
    pub typ: i32,
    /// Old record data, only applicable for UPDATE type.
    #[prost(message, optional, tag = "2")]
    pub old: ::core::option::Option<Record>,
    /// New record data.
    #[prost(message, optional, tag = "3")]
    pub new: ::core::option::Option<Record>,
    /// New record id, only applicable for INSERT type.
    #[prost(uint64, optional, tag = "4")]
    pub new_id: ::core::option::Option<u64>,
    /// Name of the endpoint that this event is from.
    #[prost(string, tag = "5")]
    pub endpoint_name: ::prost::alloc::string::String,
}
/// A record, can be thought of a row in the database table.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Record {
    /// The list of field values.
    #[prost(message, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<Value>,
    /// Records with same primary key will have increasing version.
    #[prost(uint32, tag = "2")]
    pub version: u32,
}
/// A record with its id in cache.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordWithId {
    /// The record id.
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// The record data.
    #[prost(message, optional, tag = "2")]
    pub record: ::core::option::Option<Record>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SchemaEvent {
    #[prost(string, tag = "1")]
    pub endpoint: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub version: u64,
    #[prost(int32, repeated, tag = "3")]
    pub primary_index: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, repeated, tag = "4")]
    pub fields: ::prost::alloc::vec::Vec<FieldDefinition>,
}
/// `FieldDefinition` defines a field in a schema.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldDefinition {
    /// The field type.
    #[prost(enumeration = "Type", tag = "1")]
    pub typ: i32,
    /// The field name.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// Whether the field is nullable.
    #[prost(bool, tag = "3")]
    pub nullable: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PointType {
    #[prost(double, tag = "1")]
    pub x: f64,
    #[prost(double, tag = "2")]
    pub y: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DurationType {
    /// up to u128
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
    /// nanoseconds by default
    #[prost(string, tag = "2")]
    pub time_unit: ::prost::alloc::string::String,
}
/// rust-decimal as a message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RustDecimal {
    /// the lo, mid, hi, and flags fields contain the representation of the Decimal
    /// value as a 96-bit integer
    #[prost(uint32, tag = "1")]
    pub scale: u32,
    #[prost(uint32, tag = "2")]
    pub lo: u32,
    #[prost(uint32, tag = "3")]
    pub mid: u32,
    #[prost(uint32, tag = "4")]
    pub hi: u32,
    #[prost(bool, tag = "5")]
    pub negative: bool,
}
/// A field value.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    /// A field value.
    #[prost(
        oneof = "value::Value",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14"
    )]
    pub value: ::core::option::Option<value::Value>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    /// A field value.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// Unsigned 64 bit integer.
        #[prost(uint64, tag = "1")]
        UintValue(u64),
        /// Unsigned 128 bit integer.
        #[prost(string, tag = "2")]
        Uint128Value(::prost::alloc::string::String),
        /// Signed 64 bit integer.
        #[prost(int64, tag = "3")]
        IntValue(i64),
        /// Signed 128 bit integer.
        #[prost(string, tag = "4")]
        Int128Value(::prost::alloc::string::String),
        /// 64 bit floating point number.
        #[prost(double, tag = "5")]
        FloatValue(f64),
        /// Boolean.
        #[prost(bool, tag = "6")]
        BoolValue(bool),
        /// UTF-8 string.
        #[prost(string, tag = "7")]
        StringValue(::prost::alloc::string::String),
        /// Binary data.
        #[prost(bytes, tag = "8")]
        BytesValue(::prost::alloc::vec::Vec<u8>),
        /// Decimal value.
        #[prost(message, tag = "9")]
        DecimalValue(super::RustDecimal),
        /// DateTime & Timestamp.
        #[prost(message, tag = "10")]
        TimestampValue(::prost_types::Timestamp),
        /// ISO 8601 calendar date without timezone.
        #[prost(string, tag = "11")]
        DateValue(::prost::alloc::string::String),
        /// Point type.
        #[prost(message, tag = "12")]
        PointValue(super::PointType),
        /// Duration type.
        #[prost(message, tag = "13")]
        DurationValue(super::DurationType),
        /// JSON type.
        #[prost(message, tag = "14")]
        JsonValue(::prost_types::Value),
    }
}
/// Event types that user can subscribe.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EventType {
    /// All events.
    All = 0,
    /// Only INSERT events.
    InsertOnly = 1,
    /// Only UPDATE events.
    UpdateOnly = 2,
    /// Only DELETE events.
    DeleteOnly = 3,
}
impl EventType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EventType::All => "ALL",
            EventType::InsertOnly => "INSERT_ONLY",
            EventType::UpdateOnly => "UPDATE_ONLY",
            EventType::DeleteOnly => "DELETE_ONLY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ALL" => Some(Self::All),
            "INSERT_ONLY" => Some(Self::InsertOnly),
            "UPDATE_ONLY" => Some(Self::UpdateOnly),
            "DELETE_ONLY" => Some(Self::DeleteOnly),
            _ => None,
        }
    }
}
/// The event types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperationType {
    /// INSERT operation.
    Insert = 0,
    /// DELETE operation.
    Delete = 1,
    /// UPDATE operation.
    Update = 2,
}
impl OperationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OperationType::Insert => "INSERT",
            OperationType::Delete => "DELETE",
            OperationType::Update => "UPDATE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INSERT" => Some(Self::Insert),
            "DELETE" => Some(Self::Delete),
            "UPDATE" => Some(Self::Update),
            _ => None,
        }
    }
}
/// Supported data types in Dozer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Type {
    /// Unsigned 64 bit integer.
    UInt = 0,
    /// Unsigned 128 bit integer.
    U128 = 1,
    /// Signed 64 bit integer.
    Int = 2,
    /// Signed 128 bit integer.
    I128 = 3,
    /// 64 bit floating point number.
    Float = 4,
    /// Boolean.
    Boolean = 5,
    /// UTF-8 string.
    String = 6,
    /// UTF-8 string.
    Text = 7,
    /// Binary data.
    Binary = 8,
    /// Decimal number.
    Decimal = 9,
    /// ISO 8601 combined date and time with time zone.
    Timestamp = 10,
    /// ISO 8601 calendar date without timezone.
    Date = 11,
    /// JSON data.
    Json = 12,
    /// Geo Point type.
    Point = 13,
    /// Duration type.
    Duration = 14,
}
impl Type {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Type::UInt => "UInt",
            Type::U128 => "U128",
            Type::Int => "Int",
            Type::I128 => "I128",
            Type::Float => "Float",
            Type::Boolean => "Boolean",
            Type::String => "String",
            Type::Text => "Text",
            Type::Binary => "Binary",
            Type::Decimal => "Decimal",
            Type::Timestamp => "Timestamp",
            Type::Date => "Date",
            Type::Json => "Json",
            Type::Point => "Point",
            Type::Duration => "Duration",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UInt" => Some(Self::UInt),
            "U128" => Some(Self::U128),
            "Int" => Some(Self::Int),
            "I128" => Some(Self::I128),
            "Float" => Some(Self::Float),
            "Boolean" => Some(Self::Boolean),
            "String" => Some(Self::String),
            "Text" => Some(Self::Text),
            "Binary" => Some(Self::Binary),
            "Decimal" => Some(Self::Decimal),
            "Timestamp" => Some(Self::Timestamp),
            "Date" => Some(Self::Date),
            "Json" => Some(Self::Json),
            "Point" => Some(Self::Point),
            "Duration" => Some(Self::Duration),
            _ => None,
        }
    }
}
