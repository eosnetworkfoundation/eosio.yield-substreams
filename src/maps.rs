use crate::{pb::eosio::r#yield::types::v1::{Actions, RewardsLog}, abi::Rewardslog};

#[substreams::handlers::map]
fn map_actions(block: substreams_antelope::Block) -> Result<Actions, substreams::errors::Error> {
    Ok(Actions {
        rewardslogs: block.actions::<Rewardslog>(&["eosio.yield"])
            .map(|(action, trx)| RewardsLog {
                protocol: action.protocol,
                category: action.category,
                period: 0,
                period_interval: action.period_interval,
                tvl: action.tvl,
                usd: action.usd,
                rewards: action.rewards,
                balance: action.balance,
            })
            .collect(),
    })
}