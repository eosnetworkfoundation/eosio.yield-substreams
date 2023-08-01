// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Actions {
    #[prost(message, repeated, tag="1")]
    pub rewardslogs: ::prost::alloc::vec::Vec<RewardsLog>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardsLog {
    /// Name
    #[prost(string, tag="1")]
    pub protocol: ::prost::alloc::string::String,
    /// Name
    #[prost(string, tag="2")]
    pub category: ::prost::alloc::string::String,
    /// TimePointSec
    #[prost(int32, tag="3")]
    pub period: i32,
    /// Uint32
    #[prost(uint32, tag="4")]
    pub period_interval: u32,
    /// Asset
    #[prost(string, tag="5")]
    pub tvl: ::prost::alloc::string::String,
    /// Asset
    #[prost(string, tag="6")]
    pub usd: ::prost::alloc::string::String,
    /// Asset
    #[prost(string, tag="7")]
    pub rewards: ::prost::alloc::string::String,
    /// Asset
    #[prost(string, tag="8")]
    pub balance: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
