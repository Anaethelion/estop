use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mem2 {
    pub total_virtual_in_bytes: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cpu2 {
    pub percent: i64,
    pub total_in_millis: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Process {
    pub cpu: Cpu2,
    pub max_file_descriptors: i64,
    pub mem: Mem2,
    pub open_file_descriptors: i64,
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Swap {
    pub free_in_bytes: i64,
    pub total_in_bytes: i64,
    pub used_in_bytes: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mem1 {
    pub adjusted_total_in_bytes: i64,
    pub free_in_bytes: i64,
    pub free_percent: i64,
    pub total_in_bytes: i64,
    pub used_in_bytes: i64,
    pub used_percent: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cpu1 {
    pub percent: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Memory1 {
    pub control_group: String,
    pub limit_in_bytes: String,
    pub usage_in_bytes: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cpuacct {
    pub control_group: String,
    pub usage_nanos: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Stat {
    pub number_of_elapsed_periods: i64,
    pub number_of_times_throttled: i64,
    pub time_throttled_nanos: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cpu {
    pub cfs_period_micros: i64,
    pub cfs_quota_micros: i64,
    pub control_group: String,
    pub stat: Stat,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cgroup {
    pub cpu: Cpu,
    pub cpuacct: Cpuacct,
    pub memory: Memory1,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Os {
    pub cgroup: Cgroup,
    pub cpu: Cpu1,
    pub mem: Mem1,
    pub swap: Swap,
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Threads {
    pub count: i64,
    pub peak_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Struct47 {
    pub max_in_bytes: i64,
    pub peak_max_in_bytes: i64,
    pub peak_used_in_bytes: i64,
    pub used_in_bytes: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mem {
    pub heap_committed_in_bytes: i64,
    pub heap_max_in_bytes: i64,
    pub heap_used_in_bytes: i64,
    pub heap_used_percent: i64,
    pub non_heap_committed_in_bytes: i64,
    pub non_heap_used_in_bytes: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Struct46 {
    pub collection_count: i64,
    pub collection_time_in_millis: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Collectors {
    #[serde(rename = "G1 Concurrent GC")]
    pub g1_concurrent_gc: Struct46,
    pub old: Struct46,
    pub young: Struct46,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Gc {
    pub collectors: Collectors,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Classes {
    pub current_loaded_count: i64,
    pub total_loaded_count: i64,
    pub total_unloaded_count: i64,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Jvm {
    pub classes: Classes,
    pub gc: Gc,
    pub mem: Mem,
    pub threads: Threads,
    pub timestamp: i64,
    pub uptime_in_millis: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Warmer {
    pub current: i64,
    pub total: i64,
    pub total_time_in_millis: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Translog {
    pub earliest_last_modified_age: i64,
    pub operations: i64,
    pub size_in_bytes: i64,
    pub uncommitted_operations: i64,
    pub uncommitted_size_in_bytes: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Store {
    pub reserved_in_bytes: i64,
    pub size_in_bytes: i64,
    pub total_data_set_size_in_bytes: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShardStats {
    pub total_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Segments {
    pub count: i64,
    pub doc_values_memory_in_bytes: i64,
    pub fixed_bit_set_memory_in_bytes: i64,
    pub index_writer_memory_in_bytes: i64,
    pub max_unsafe_auto_id_timestamp: i64,
    pub memory_in_bytes: i64,
    pub norms_memory_in_bytes: i64,
    pub points_memory_in_bytes: i64,
    pub stored_fields_memory_in_bytes: i64,
    pub term_vectors_memory_in_bytes: i64,
    pub terms_memory_in_bytes: i64,
    pub version_map_memory_in_bytes: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Search {
    pub fetch_current: i64,
    pub fetch_failure: i64,
    pub fetch_time_in_millis: i64,
    pub fetch_total: i64,
    pub open_contexts: i64,
    pub query_current: i64,
    pub query_failure: i64,
    pub query_time_in_millis: i64,
    pub query_total: i64,
    pub scroll_current: i64,
    pub scroll_time_in_millis: i64,
    pub scroll_total: i64,
    pub suggest_current: i64,
    pub suggest_time_in_millis: i64,
    pub suggest_total: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestCache {
    pub evictions: i64,
    pub hit_count: i64,
    pub memory_size_in_bytes: i64,
    pub miss_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Refresh {
    pub external_total: i64,
    pub external_total_time_in_millis: i64,
    pub listeners: i64,
    pub total: i64,
    pub total_time_in_millis: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recovery {
    pub current_as_source: i64,
    pub current_as_target: i64,
    pub throttle_time_in_millis: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryCache {
    pub cache_count: i64,
    pub cache_size: i64,
    pub evictions: i64,
    pub hit_count: i64,
    pub memory_size_in_bytes: i64,
    pub miss_count: i64,
    pub total_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Merges {
    pub current: i64,
    pub current_docs: i64,
    pub current_size_in_bytes: i64,
    pub total: i64,
    pub total_auto_throttle_in_bytes: i64,
    pub total_docs: i64,
    pub total_size_in_bytes: i64,
    pub total_stopped_time_in_millis: i64,
    pub total_throttled_time_in_millis: i64,
    pub total_time_in_millis: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Indexing {
    pub delete_current: i64,
    pub delete_time_in_millis: i64,
    pub delete_total: i64,
    pub index_current: i64,
    pub index_failed: i64,
    pub index_time_in_millis: i64,
    pub index_total: i64,
    pub is_throttled: bool,
    pub noop_update_total: i64,
    pub throttle_time_in_millis: i64,
    pub write_load: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Get {
    pub current: i64,
    pub exists_time_in_millis: i64,
    pub exists_total: i64,
    pub missing_time_in_millis: i64,
    pub missing_total: i64,
    pub time_in_millis: i64,
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Flush {
    pub periodic: i64,
    pub total: i64,
    pub total_time_excluding_waiting_on_lock_in_millis: i64,
    pub total_time_in_millis: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalOrdinals {
    pub build_time_in_millis: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fielddata1 {
    pub evictions: i64,
    pub global_ordinals: GlobalOrdinals,
    pub memory_size_in_bytes: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Docs {
    pub count: i64,
    pub deleted: i64,
    pub total_size_in_bytes: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Struct19 {
    pub value_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Completion {
    pub size_in_bytes: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bulk {
    pub avg_size_in_bytes: i64,
    pub avg_time_in_millis: i64,
    pub total_operations: i64,
    pub total_size_in_bytes: i64,
    pub total_time_in_millis: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Total1 {
    pub all_in_bytes: i64,
    pub combined_coordinating_and_primary_in_bytes: i64,
    pub coordinating_in_bytes: i64,
    pub coordinating_rejections: i64,
    pub primary_document_rejections: i64,
    pub primary_in_bytes: i64,
    pub primary_rejections: i64,
    pub replica_in_bytes: i64,
    pub replica_rejections: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Current {
    pub all_in_bytes: i64,
    pub combined_coordinating_and_primary_in_bytes: i64,
    pub coordinating_in_bytes: i64,
    pub primary_in_bytes: i64,
    pub replica_in_bytes: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Memory {
    pub current: Current,
    pub limit_in_bytes: i64,
    pub total: Total1,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IndexingPressure {
    pub memory: Memory,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Total {
    pub available_in_bytes: i64,
    pub free_in_bytes: i64,
    pub total_in_bytes: i64,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Fs {
    pub timestamp: i64,
    pub total: Total,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Unchanged {
    pub computation_time_millis: i64,
    pub count: i64,
    pub notification_time_millis: i64,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Struct1 {
    pub cumulative_execution_count: i64,
    pub cumulative_execution_time_millis: i64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fielddata {
    pub estimated_size: String,
    pub estimated_size_in_bytes: i64,
    pub limit_size: String,
    pub limit_size_in_bytes: i64,
    pub overhead: f64,
    pub tripped: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Struct {
    pub estimated_size: String,
    pub estimated_size_in_bytes: i64,
    pub limit_size: String,
    pub limit_size_in_bytes: i64,
    pub overhead: i64,
    pub tripped: i64,
}

pub type Attributes = HashMap<String, String>;

#[derive(Debug, Serialize, Deserialize)]
pub struct OuebKbhkr82apri8mmUakq1 {
    pub avg_queue_size: i64,
    pub avg_response_time_ns: i64,
    pub avg_service_time_ns: i64,
    pub outgoing_searches: i64,
    pub rank: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub attributes: Attributes,
    pub fs: Fs,
    pub host: String,
    pub indexing_pressure: IndexingPressure,
    pub ip: String,
    pub jvm: Jvm,
    pub name: String,
    pub os: Os,
    pub process: Process,
    pub roles: Vec<String>,
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct _Nodes {
    pub failed: i64,
    pub successful: i64,
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodesStats {
    #[serde(rename = "_nodes")]
    pub _nodes: _Nodes,
    pub cluster_name: String,
    pub nodes: HashMap<String, NodeInfo>,
}