use substreams::errors::Error;
use substreams_sink_prometheus::{PrometheusOperations, Counter};
use crate::{abi, pb::eosio::r#yield::types::v1::Actions};

#[substreams::handlers::map]
pub fn prom_out(actions: Actions) -> Result<PrometheusOperations, Error> {
    let mut prom_out = PrometheusOperations::default();
    Ok(prom_out)
}