use crate::elasticsearch::cluster::ClusterStats;
use crate::elasticsearch::indices::IndicesStats;
use crate::elasticsearch::nodes::NodeInfo;
use crate::elasticsearch::{get_cluster_stats, get_indices_stats, get_node_stats};
use crate::trace_dbg;
use anyhow::{anyhow, Error, Result};
use elasticsearch::Elasticsearch;
use ordermap::OrderMap;
use std::ops::Index;

pub struct App {
    pub elasticsearch_client: Elasticsearch,
    pub cluster_stats: Option<ClusterStats>,
    pub indices_stats: Option<IndicesStats>,
    pub node_stats: OrderMap<String, Vec<NodeInfo>>,
    pub selected_tab: usize,
    pub selected_node: usize,
    pub cpu_usage_history: Vec<(f64, f64)>,
    pub memory_usage_history: Vec<(f64, f64)>,
    pub window: [f64; 2],
}

impl App {
    pub fn new(client: Elasticsearch) -> App {
        App {
            elasticsearch_client: client,
            cluster_stats: None,
            indices_stats: None,
            node_stats: OrderMap::new(),
            selected_tab: 0,
            selected_node: 0,
            cpu_usage_history: Vec::with_capacity(60),
            memory_usage_history: Vec::with_capacity(60),
            window: [0.0, 60.0],
        }
    }


    pub async fn on_tick(&mut self) -> Result<(), Error> {
        // Update cluster stats
        match get_cluster_stats(&self.elasticsearch_client).await {
            Ok(stats) => self.cluster_stats = {
                Some(stats)
            },
            Err(e) => {
                trace_dbg!(&e);
            }
        }

        match get_indices_stats(&self.elasticsearch_client).await {
            Ok(stats) => self.indices_stats = Some(stats),
            Err(e) => {
                trace_dbg!(&e);
            }
        }

        // Update node stats
        match get_node_stats(&self.elasticsearch_client).await {
            Ok(stats) => {
                for (name, nodeinfo) in stats {
                    let nodesinfo = self.node_stats.entry(name).and_modify(|e| e.push(nodeinfo.clone())).or_insert_with(|| vec![nodeinfo]);
                    if nodesinfo.len() > 60 {
                        nodesinfo.remove(0);
                    }
                }

                let selected_node = self.node_stats.index(self.selected_node);


                // Update CPU usage history
                self.cpu_usage_history = selected_node
                    .iter()
                    .enumerate()
                    .map(|(idx, nodeinfo)| {
                        (idx as f64, nodeinfo.os.cpu.percent as f64)
                    })
                    .collect();

                // Update memory usage history
                self.memory_usage_history = selected_node
                    .iter()
                    .enumerate()
                    .map(|(idx, nodeinfo)| {
                        (idx as f64, nodeinfo.os.mem.used_percent as f64)
                    })
                    .collect();


                self.window[0] = self.cpu_usage_history.len() as f64 - 60.0;
                self.window[1] = self.cpu_usage_history.len() as f64;
            }
            Err(e) => Err(anyhow!("Failed to get node stats: {:?}", e))?,
        }

        Ok(())
    }

    pub fn on_up(&mut self) {
        if self.selected_tab == 1 {
            self.selected_node = self.selected_node.saturating_sub(1);
        }
    }

    pub fn on_down(&mut self) {
        if self.selected_tab == 1 {
            self.selected_node = (self.selected_node + 1).min(self.node_stats.len() - 1);
        }
    }

    pub fn on_left(&mut self) {
        self.selected_tab = self.selected_tab.saturating_sub(1);
    }

    pub fn on_right(&mut self) {
        self.selected_tab = (self.selected_tab + 1).min(2);
    }
}
