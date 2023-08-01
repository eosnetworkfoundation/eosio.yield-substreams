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
    /// trx
    ///
    /// Checksum256
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub block_num: u64,
    /// TimePointSec
    #[prost(int64, tag="3")]
    pub timestamp: i64,
    /// action
    ///
    /// Name
    #[prost(string, tag="4")]
    pub protocol: ::prost::alloc::string::String,
    /// Name
    #[prost(string, tag="5")]
    pub category: ::prost::alloc::string::String,
    /// TimePointSec
    #[prost(string, tag="6")]
    pub period: ::prost::alloc::string::String,
    /// Uint32
    #[prost(uint32, tag="7")]
    pub period_interval: u32,
    /// Asset
    #[prost(string, tag="8")]
    pub tvl: ::prost::alloc::string::String,
    /// Asset
    #[prost(string, tag="9")]
    pub usd: ::prost::alloc::string::String,
    /// Asset
    #[prost(string, tag="10")]
    pub rewards: ::prost::alloc::string::String,
    /// Asset
    #[prost(string, tag="11")]
    pub balance: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
