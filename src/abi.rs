pub mod types {
    use substreams_antelope::types::*;
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct ConfigRow {
        pub annual_rate: Uint16,
        pub min_tvl_report: Asset,
        pub max_tvl_report: Asset,
        pub rewards: ExtendedSymbol,
        pub oracle_contract: Name,
        pub admin_contract: Name,
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct ExtendedSymbol {
        pub sym: Symbol,
        pub contract: Name,
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct PairNameString {
        pub first: Name,
        pub second: String,
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct ProtocolsRow {
        pub protocol: Name,
        pub status: Name,
        pub category: Name,
        pub contracts: Vec<Name>,
        pub evm_contracts: Vec<String>,
        pub tvl: Asset,
        pub usd: Asset,
        pub balance: ExtendedAsset,
        pub metadata: Vec<PairNameString>,
        pub created_at: TimePointSec,
        pub updated_at: TimePointSec,
        pub claimed_at: TimePointSec,
        pub period_at: TimePointSec,
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct StateRow {
        pub active_protocols: Vec<Name>,
    }
}
pub mod actions {
    use substreams_antelope::types::*;
    use super::types::*;
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Approve {
        pub protocol: Name,
    }
    impl substreams_antelope::action::Action for Approve {
        const NAME: &'static str = "approve";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Claim {
        pub protocol: Name,
        pub receiver: Name,
        pub evm_receiver: String,
    }
    impl substreams_antelope::action::Action for Claim {
        const NAME: &'static str = "claim";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Claimlog {
        pub protocol: Name,
        pub category: Name,
        pub receiver: Name,
        pub evm_receiver: String,
        pub claimed: Asset,
        pub balance: Asset,
    }
    impl substreams_antelope::action::Action for Claimlog {
        const NAME: &'static str = "claimlog";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Contractslog {
        pub protocol: Name,
        pub status: Name,
        pub contracts: Vec<Name>,
        pub evm: Vec<String>,
    }
    impl substreams_antelope::action::Action for Contractslog {
        const NAME: &'static str = "contractslog";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Createlog {
        pub protocol: Name,
        pub status: Name,
        pub category: Name,
        pub metadata: Vec<PairNameString>,
    }
    impl substreams_antelope::action::Action for Createlog {
        const NAME: &'static str = "createlog";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Deny {
        pub protocol: Name,
    }
    impl substreams_antelope::action::Action for Deny {
        const NAME: &'static str = "deny";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Eraselog {
        pub protocol: Name,
    }
    impl substreams_antelope::action::Action for Eraselog {
        const NAME: &'static str = "eraselog";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Init {
        pub rewards: ExtendedSymbol,
        pub oracle_contract: Name,
        pub admin_contract: Name,
    }
    impl substreams_antelope::action::Action for Init {
        const NAME: &'static str = "init";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Metadatalog {
        pub protocol: Name,
        pub status: Name,
        pub category: Name,
        pub metadata: Vec<PairNameString>,
    }
    impl substreams_antelope::action::Action for Metadatalog {
        const NAME: &'static str = "metadatalog";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Regprotocol {
        pub protocol: Name,
        pub category: Name,
        pub metadata: Vec<PairNameString>,
    }
    impl substreams_antelope::action::Action for Regprotocol {
        const NAME: &'static str = "regprotocol";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Report {
        pub protocol: Name,
        pub period: TimePointSec,
        pub period_interval: Uint32,
        pub tvl: Asset,
        pub usd: Asset,
    }
    impl substreams_antelope::action::Action for Report {
        const NAME: &'static str = "report";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Rewardslog {
        pub protocol: Name,
        pub category: Name,
        pub period: TimePointSec,
        pub period_interval: Uint32,
        pub tvl: Asset,
        pub usd: Asset,
        pub rewards: Asset,
        pub balance: Asset,
    }
    impl substreams_antelope::action::Action for Rewardslog {
        const NAME: &'static str = "rewardslog";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setcategory {
        pub protocol: Name,
        pub category: Name,
    }
    impl substreams_antelope::action::Action for Setcategory {
        const NAME: &'static str = "setcategory";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setcontracts {
        pub protocol: Name,
        pub contracts: Vec<Name>,
        pub evm_contracts: Vec<String>,
    }
    impl substreams_antelope::action::Action for Setcontracts {
        const NAME: &'static str = "setcontracts";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setmetadata {
        pub protocol: Name,
        pub metadata: Vec<PairNameString>,
    }
    impl substreams_antelope::action::Action for Setmetadata {
        const NAME: &'static str = "setmetadata";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setmetakey {
        pub protocol: Name,
        pub key: Name,
        pub value: String,
    }
    impl substreams_antelope::action::Action for Setmetakey {
        const NAME: &'static str = "setmetakey";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setrate {
        pub annual_rate: Int16,
        pub min_tvl_report: Asset,
        pub max_tvl_report: Asset,
    }
    impl substreams_antelope::action::Action for Setrate {
        const NAME: &'static str = "setrate";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Statuslog {
        pub protocol: Name,
        pub status: Name,
    }
    impl substreams_antelope::action::Action for Statuslog {
        const NAME: &'static str = "statuslog";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Unregister {
        pub protocol: Name,
    }
    impl substreams_antelope::action::Action for Unregister {
        const NAME: &'static str = "unregister";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
}