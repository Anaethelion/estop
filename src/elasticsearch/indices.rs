use std::cmp::Ordering;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ShardPath {
    pub data_path: String,
    pub is_custom_data_path: bool,
    pub state_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct SeqNo {
    pub global_checkpoint: i64,
    pub local_checkpoint: i64,
    pub max_seq_no: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Struct4 {
    pub id: String,
    pub retaining_seq_no: i64,
    pub source: String,
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize)]
pub struct RetentionLeases {
    pub leases: Vec<Struct4>,
    pub primary_term: i64,
    pub version: i64,
}

#[derive(Serialize, Deserialize)]
pub struct UserData {
    pub es_version: String,
    pub history_uuid: String,
    pub local_checkpoint: String,
    pub max_seq_no: String,
    pub max_unsafe_auto_id_timestamp: String,
    pub translog_uuid: String,
}

#[derive(Serialize, Deserialize)]
pub struct Commit {
    pub generation: i64,
    pub id: String,
    pub num_docs: i64,
    pub user_data: UserData,
}

#[derive(Serialize, Deserialize)]
pub struct Struct3 {
    pub bulk: Bulk,
    pub commit: Commit,
    pub completion: Completion,
    pub dense_vector: Struct1,
    pub docs: Docs,
    pub fielddata: Fielddata,
    pub flush: Flush,
    pub get: Get,
    pub indexing: Indexing,
    pub merges: Merges,
    pub query_cache: QueryCache,
    pub recovery: Recovery,
    pub refresh: Refresh,
    pub request_cache: RequestCache,
    pub retention_leases: RetentionLeases,
    pub search: Search,
    pub search_idle: bool,
    pub search_idle_time: i64,
    pub segments: Segments,
    pub seq_no: SeqNo,
    pub shard_path: ShardPath,
    pub shard_stats: ShardStats,
    pub sparse_vector: Struct1,
    pub store: Store,
    pub translog: Translog,
    pub warmer: Warmer,
}

#[derive(Serialize, Deserialize)]
pub struct Index {
    pub health: String,
    pub primaries: Struct,
    pub status: String,
    pub total: Struct,
    pub uuid: String,
}

impl Ord for Index {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total.store.total_data_set_size_in_bytes.cmp(&other.total.store.total_data_set_size_in_bytes)
    }
}

impl PartialOrd for Index {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Index {
    fn eq(&self, other: &Self) -> bool {
        self.total.store.total_data_set_size_in_bytes == other.total.store.total_data_set_size_in_bytes
    }
}

impl Eq for Index {}

#[derive(Serialize, Deserialize)]
pub struct Shards {
    pub failed: i64,
    pub successful: i64,
    pub total: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Warmer {
    pub current: i64,
    pub total: i64,
    pub total_time_in_millis: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Translog {
    pub earliest_last_modified_age: i64,
    pub operations: i64,
    pub size_in_bytes: i64,
    pub uncommitted_operations: i64,
    pub uncommitted_size_in_bytes: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Store {
    pub reserved_in_bytes: i64,
    pub size_in_bytes: i64,
    pub total_data_set_size_in_bytes: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ShardStats {
    pub total_count: i64,
}

#[derive(Serialize, Deserialize)]
pub struct FileSizes {}

#[derive(Serialize, Deserialize)]
pub struct Segments {
    pub count: i64,
    pub doc_values_memory_in_bytes: i64,
    pub file_sizes: FileSizes,
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

#[derive(Serialize, Deserialize)]
pub struct Search {
    pub fetch_current: i64,
    pub fetch_failure: Option<i64>,
    pub fetch_time_in_millis: i64,
    pub fetch_total: i64,
    pub open_contexts: i64,
    pub query_current: i64,
    pub query_failure: Option<i64>,
    pub query_time_in_millis: i64,
    pub query_total: i64,
    pub scroll_current: i64,
    pub scroll_time_in_millis: i64,
    pub scroll_total: i64,
    pub suggest_current: i64,
    pub suggest_time_in_millis: i64,
    pub suggest_total: i64,
}

#[derive(Serialize, Deserialize)]
pub struct RequestCache {
    pub evictions: i64,
    pub hit_count: i64,
    pub memory_size_in_bytes: i64,
    pub miss_count: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Refresh {
    pub external_total: i64,
    pub external_total_time_in_millis: i64,
    pub listeners: i64,
    pub total: i64,
    pub total_time_in_millis: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Recovery {
    pub current_as_source: i64,
    pub current_as_target: i64,
    pub throttle_time_in_millis: i64,
}

#[derive(Serialize, Deserialize)]
pub struct QueryCache {
    pub cache_count: i64,
    pub cache_size: i64,
    pub evictions: i64,
    pub hit_count: i64,
    pub memory_size_in_bytes: i64,
    pub miss_count: i64,
    pub total_count: i64,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct Get {
    pub current: i64,
    pub exists_time_in_millis: i64,
    pub exists_total: i64,
    pub missing_time_in_millis: i64,
    pub missing_total: i64,
    pub time_in_millis: i64,
    pub total: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Flush {
    pub periodic: i64,
    pub total: i64,
    pub total_time_excluding_waiting_on_lock_in_millis: i64,
    pub total_time_in_millis: i64,
}

#[derive(Serialize, Deserialize)]
pub struct GlobalOrdinals {
    pub build_time_in_millis: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Fielddata {
    pub evictions: i64,
    pub global_ordinals: GlobalOrdinals,
    pub memory_size_in_bytes: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Docs {
    pub count: i64,
    pub deleted: i64,
    pub total_size_in_bytes: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Struct1 {
    pub value_count: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Completion {
    pub size_in_bytes: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Bulk {
    pub avg_size_in_bytes: i64,
    pub avg_time_in_millis: i64,
    pub total_operations: i64,
    pub total_size_in_bytes: i64,
    pub total_time_in_millis: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Struct {
    pub bulk: Bulk,
    pub completion: Completion,
    pub dense_vector: Struct1,
    pub docs: Docs,
    pub fielddata: Fielddata,
    pub flush: Flush,
    pub get: Get,
    pub indexing: Indexing,
    pub merges: Merges,
    pub query_cache: QueryCache,
    pub recovery: Recovery,
    pub refresh: Refresh,
    pub request_cache: RequestCache,
    pub search: Search,
    pub segments: Segments,
    pub shard_stats: ShardStats,
    pub sparse_vector: Struct1,
    pub store: Store,
    pub translog: Translog,
    pub warmer: Warmer,
}

#[derive(Serialize, Deserialize)]
pub struct All {
    pub primaries: Struct,
    pub total: Struct,
}

#[derive(Serialize, Deserialize)]
pub struct IndicesStats {
    pub _all: All,
    pub _shards: Shards,
    pub indices: HashMap<String, Index>,
}