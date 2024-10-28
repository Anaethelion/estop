use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CurrentCounts {
    pub cleanups: i64,
    pub concurrent_operations: i64,
    pub shard_snapshots: i64,
    pub snapshot_deletions: i64,
    pub snapshots: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Snapshots {
    pub current_counts: CurrentCounts,
    pub repositories: Struct,
}

#[derive(Serialize, Deserialize)]
pub struct Cpu {
    pub percent: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Process {
    pub cpu: Cpu,
    pub open_file_descriptors: Stats,
}

#[derive(Serialize, Deserialize)]
pub struct Struct11 {
    pub count: i64,
    pub flavor: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Serialize, Deserialize)]
pub struct Struct10 {
    pub count: i64,
    pub pretty_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Struct9 {
    pub count: i64,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Mem1 {
    pub adjusted_total_in_bytes: i64,
    pub free_in_bytes: i64,
    pub free_percent: i64,
    pub total_in_bytes: i64,
    pub used_in_bytes: i64,
    pub used_percent: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Struct8 {
    pub arch: String,
    pub count: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Os {
    pub allocated_processors: i64,
    pub architectures: Vec<Struct8>,
    pub available_processors: i64,
    pub mem: Mem1,
    pub names: Vec<Struct9>,
    pub pretty_names: Vec<Struct10>,
}

#[derive(Serialize, Deserialize)]
pub struct Struct6 {
    pub bundled_jdk: bool,
    pub count: i64,
    pub using_bundled_jdk: bool,
    pub version: String,
    pub vm_name: String,
    pub vm_vendor: String,
    pub vm_version: String,
}

#[derive(Serialize, Deserialize)]
pub struct Mem {
    pub heap_max_in_bytes: i64,
    pub heap_used_in_bytes: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Jvm {
    pub max_uptime_in_millis: i64,
    pub mem: Mem,
    pub threads: i64,
    pub versions: Vec<Struct6>,
}

#[derive(Serialize, Deserialize)]
pub struct Struct5 {
    pub count: i64,
    pub current: i64,
    pub failed: i64,
    pub time_in_millis: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ProcessorStats {
    pub attachment: Struct5,
    pub dot_expander: Struct5,
    pub foreach: Struct5,
    pub geoip: Struct5,
    pub gsub: Struct5,
    pub json: Struct5,
    pub pipeline: Struct5,
    pub remove: Struct5,
    pub rename: Struct5,
    pub reroute: Struct5,
    pub script: Struct5,
    pub set: Struct5,
    pub trim: Struct5,
    pub uri_parts: Struct5,
    pub user_agent: Struct5,
}

#[derive(Serialize, Deserialize)]
pub struct Ingest {
    pub number_of_pipelines: i64,
    pub processor_stats: ProcessorStats,
}

#[derive(Serialize, Deserialize)]
pub struct Total {
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

#[derive(Serialize, Deserialize)]
pub struct Current {
    pub all_in_bytes: i64,
    pub combined_coordinating_and_primary_in_bytes: i64,
    pub coordinating_in_bytes: i64,
    pub primary_in_bytes: i64,
    pub replica_in_bytes: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Memory {
    pub current: Current,
    pub limit_in_bytes: i64,
    pub total: Total,
}

#[derive(Serialize, Deserialize)]
pub struct IndexingPressure {
    pub memory: Memory,
}

#[derive(Serialize, Deserialize)]
pub struct Fs {
    pub available_in_bytes: i64,
    pub free_in_bytes: i64,
    pub total_in_bytes: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Count {
    pub coordinating_only: i64,
    pub data: i64,
    pub data_cold: i64,
    pub data_content: i64,
    pub data_frozen: i64,
    pub data_hot: i64,
    pub data_warm: i64,
    pub index: i64,
    pub ingest: i64,
    pub master: i64,
    pub ml: i64,
    pub remote_cluster_client: i64,
    pub search: i64,
    pub total: i64,
    pub transform: i64,
    pub voting_only: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Nodes {
    pub count: Count,
    pub fs: Fs,
    pub indexing_pressure: IndexingPressure,
    pub ingest: Ingest,
    pub jvm: Jvm,
    pub os: Os,
    pub packaging_types: Vec<Struct11>,
    pub process: Process,
    pub versions: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Struct4 {
    pub index_count: i64,
    pub primary_shard_count: i64,
    pub total_primary_bytes: i64,
    pub version: String,
}

#[derive(Serialize, Deserialize)]
pub struct Store {
    pub reserved_in_bytes: i64,
    pub size_in_bytes: i64,
    pub total_data_set_size_in_bytes: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Stats {
    pub avg: f64,
    pub max: f64,
    pub min: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Index {
    pub primaries: Stats,
    pub replication: Stats,
    pub shards: Stats,
}

#[derive(Serialize, Deserialize)]
pub struct Shards {
    pub index: Index,
    pub primaries: i64,
    pub replication: f64,
    pub total: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Segments {
    pub count: i64,
    pub doc_values_memory_in_bytes: i64,
    pub file_sizes: Struct,
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
pub struct Sections {
    pub aggs: Option<i64>,
    pub fields: Option<i64>,
    pub min_score: Option<i64>,
    pub query: Option<i64>,
    pub runtime_mappings: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Queries {
    pub bool: Option<i64>,
    pub constant_score: Option<i64>,
    pub exists: Option<i64>,
    pub function_score: Option<i64>,
    pub geo_bounding_box: Option<i64>,
    #[serde(rename = "match")]
    pub r#match: Option<i64>,
    pub match_all: Option<i64>,
    pub query_string: Option<i64>,
    pub range: Option<i64>,
    pub simple_query_string: Option<i64>,
    pub term: Option<i64>,
    pub terms: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Search1 {
    pub queries: Queries,
    pub sections: Sections,
    pub total: i64,
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
pub struct Mappings {
    pub total_deduplicated_field_count: i64,
    pub total_deduplicated_mapping_size_in_bytes: i64,
    pub total_field_count: i64,
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
pub struct DenseVector {
    pub value_count: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Completion {
    pub size_in_bytes: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Indices {
    pub completion: Completion,
    pub count: i64,
    pub dense_vector: DenseVector,
    pub docs: Docs,
    pub fielddata: Fielddata,
    pub mappings: Mappings,
    pub query_cache: QueryCache,
    pub search: Search1,
    pub segments: Segments,
    pub shards: Shards,
    pub sparse_vector: DenseVector,
    pub store: Store,
    pub versions: Vec<Struct4>,
}

#[derive(Serialize, Deserialize)]
pub struct Struct1 {
    pub avg: i64,
    pub max: i64,
    pub p90: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Struct {}

#[derive(Serialize, Deserialize)]
pub struct Search {
    pub remotes_per_search_avg: i64,
    pub remotes_per_search_max: i64,
    pub skipped: i64,
    pub success: i64,
    pub took: Struct1,
    pub took_mrt_false: Struct1,
    pub took_mrt_true: Struct1,
    pub total: i64,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct _Nodes {
    pub failed: i64,
    pub successful: i64,
    pub total: i64,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ClusterStats {
    pub _nodes: _Nodes,
    pub cluster_name: String,
    pub cluster_uuid: String,
    pub indices: Indices,
    pub nodes: Nodes,
    pub snapshots: Snapshots,
    pub status: String,
    pub timestamp: i64,
}


#[derive(Debug, Deserialize)]
pub enum HealthStatus {
    Green,
    Yellow,
    Red,
}