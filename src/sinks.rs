use std::collections::HashMap;

use antelope::Asset;
use substreams::errors::Error;
use substreams_sink_prometheus::{PrometheusOperations, Counter};
use crate::pb::eosio::r#yield::v1::Actions;

#[substreams::handlers::map]
pub fn prom_out(actions: Actions) -> Result<PrometheusOperations, Error> {
    let mut prom_out = PrometheusOperations::default();
    for rewardslog in actions.rewardslogs {
        let labels = HashMap::from([("protocol".to_string(), rewardslog.protocol)]);
        let mut counter = Counter::from("rewardslog").with(labels);
        let rewards = Asset::from(rewardslog.rewards.as_str());
        prom_out.push(counter.add(rewards.value()));
    }
    Ok(prom_out)
}