use crate::{pb::eosio::r#yield::v1::{Actions, RewardsLog}, abi::actions::Rewardslog};

#[substreams::handlers::map]
fn map_actions(block: substreams_antelope::Block) -> Result<Actions, substreams::errors::Error> {
    Ok(Actions {
        rewardslogs: block.actions::<Rewardslog>(&["eosio.yield"])
            .filter(|(_action, trx) | trx.receiver == "eosio.yield")
            .map(|(action, trx)| RewardsLog {
                // trx
                trx_id: trx.transaction_id.to_string(),
                block_num: trx.block_num,
                timestamp: trx.block_time.clone().unwrap().seconds,

                // action
                protocol: action.protocol,
                category: action.category,
                period: action.period,
                period_interval: action.period_interval,
                tvl: action.tvl,
                usd: action.usd,
                rewards: action.rewards,
                balance: action.balance,
            })
            .collect(),
    })
}