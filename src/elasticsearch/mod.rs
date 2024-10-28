use std::collections::HashMap;
use elasticsearch::cluster::ClusterStatsParts;
use elasticsearch::nodes::NodesStatsParts;
use elasticsearch::{Elasticsearch, Error};
use crate::elasticsearch::nodes::{NodeInfo, NodesStats};
use crate::elasticsearch::cluster::ClusterStats;
use crate::elasticsearch::indices::IndicesStats;
use crate::trace_dbg;

pub mod nodes;
pub mod cluster;
pub(crate) mod indices;

pub async fn get_cluster_stats(client: &Elasticsearch) -> Result<ClusterStats, Error> {
    let response = client
        .cluster()
        .stats(ClusterStatsParts::None)
        .send()
        .await?;

    match response.json::<ClusterStats>().await {
        Ok(stats) => Ok(stats),
        Err(e) => {
            trace_dbg!(&e);
            Err(e)
        }
    }
}

pub async fn get_node_stats(client: &Elasticsearch) -> Result<HashMap<String, NodeInfo>, Error> {
    let response = client
        .nodes()
        .stats(NodesStatsParts::None)
        .send()
        .await?;

    let json = response.json().await?;
    let s: NodesStats = serde_json::from_value(json)?;

    Ok(s.nodes)
}

pub async fn get_indices_stats(client: &Elasticsearch) -> Result<IndicesStats, Error> {
    let response = client
        .indices()
        .stats(elasticsearch::indices::IndicesStatsParts::None)
        .send()
        .await?;

    let json = response.json().await?;
    let s: IndicesStats = serde_json::from_value(json)?;

    Ok(s)
}