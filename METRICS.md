# Metrics exposed

| Name | Description | Labels | Type |
| ---  | ----------- | --- | ---- |
| `fdb_client_coordinator_reachable` | Whether the coordinator is reachable | `["address"]` | GAUGE |
| `fdb_client_coordinators_count` | Number of coordinators registered in client fdb.cluster | `null` | GAUGE |
| `fdb_client_messages_count` | Number of messages available when fetching status | `null` | GAUGE |
| `fdb_client_quorum_reachable` | The quorum of coordinators is reachable | `null` | GAUGE |
| `fdb_client_timestamp` | Client timestamp when last fetched | `null` | GAUGE |
| `fdb_cluster_average_partition_size_bytes` | Average size for a partition in the cluster | `null` | GAUGE |
| `fdb_cluster_generation_count` | Number of generations | `null` | GAUGE |
| `fdb_cluster_healthy` | Whether the cluster is healthy or not | `null` | GAUGE |
| `fdb_cluster_latency_commit_seconds` | Time in seconds to commit a transaction | `null` | GAUGE |
| `fdb_cluster_latency_read_seconds` | Time in seconds to read | `null` | GAUGE |
| `fdb_cluster_latency_transaction_start_seconds` | Time in seconds to start a transaction | `null` | GAUGE |
| `fdb_cluster_least_space_log_server_bytes` | Value of the log server with least space available | `null` | GAUGE |
| `fdb_cluster_least_space_storage_server_bytes` | Value of the storage server with least space avaiable | `null` | GAUGE |
| `fdb_cluster_machine_contributing_workers_count` | Number of process workers on the machine | `["datacenter_id","machine_id"]` | GAUGE |
| `fdb_cluster_machine_excluded` | Machine is being excluded of the cluster | `["datacenter_id","machine_id"]` | GAUGE |
| `fdb_cluster_machine_memory_committed_bytes` | Estimated number of bytes of memory not available on the machine | `["datacenter_id","machine_id"]` | GAUGE |
| `fdb_cluster_machine_memory_free_bytes` | Estimated number of bytes of memory that are available on the machine without swapping | `["datacenter_id","machine_id"]` | GAUGE |
| `fdb_cluster_machine_memory_total_bytes` | Estimated number of total physical RAM | `["datacenter_id","machine_id"]` | GAUGE |
| `fdb_cluster_machine_network_received_megabits` | Received megabits | `["datacenter_id","machine_id"]` | GAUGE |
| `fdb_cluster_machine_network_sent_megabits` | Sent megabits | `["datacenter_id","machine_id"]` | GAUGE |
| `fdb_cluster_machine_network_tcp_segment_retransmitted` | Number of TCP segments that have been retransmitted | `["datacenter_id","machine_id"]` | GAUGE |
| `fdb_cluster_machines_count` | Number of machines available in the cluster | `null` | GAUGE |
| `fdb_cluster_moving_data_in_flight_bytes` | Data in flight | `null` | GAUGE |
| `fdb_cluster_moving_data_in_queue_bytes` | Data waiting to be transferred | `null` | GAUGE |
| `fdb_cluster_partition_count` | Number of partitions | `null` | GAUGE |
| `fdb_cluster_process_busy` | Busy of the process (value between 0.0 and 1.1) | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_cpu_usage` | Current usage of CPU (between 0 and 1) | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_disk_busy` | Disk is being busy (0.0 to 1.0 value) | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_disk_free_bytes` | Bytes available on the disk used by process | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_disk_reads_count` | Number of reads on the disk | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_disk_reads_frequency` | Frequency of reads on the disk | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_disk_reads_sectors` | N/A | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_disk_total_bytes` | Bytes total on the disk used by process | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_disk_writes_count` | Number of writes on the disk | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_disk_writes_frequency` | Frequency of writes on the disk | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_disk_writes_sectors` | N/A | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_excluded` | Process is being excluded by the cluster | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_memory_available_bytes` | Available bytes for the current process | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_memory_limit_bytes` | Limiting bytes for the current process | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_memory_rss_bytes` | N/A | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_memory_unused_allocated_bytes` | N/A | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_memory_used_bytes` | N/A | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_network_connection_errors_freq` | Frequency of connection errors | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_network_connections_closed` | Frequency of connection closed | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_network_connections_established` | Frequency of connection established | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_network_received_megabits` | Megabits received on network | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_network_sent_megabits` | Megabits sent on network | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_bytes_queried_counter` | Frequency of write storage server operations in bytes | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_bytes_queried_hz` | Frequency of write storage server operations in bytes | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_bytes_queried_roughness` | Frequency of write storage server operations in bytes | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_batching_window_count` | Commit batching window size latency  | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_batching_window_max` | Commit batching window size latency  | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_batching_window_mean` | Commit batching window size latency  | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_batching_window_median` | Commit batching window size latency  | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_batching_window_min` | Commit batching window size latency  | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_batching_window_p25` | Commit batching window size latency  | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_batching_window_p90` | Commit batching window size latency  | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_batching_window_p95` | Commit batching window size latency  | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_batching_window_p99` | Commit batching window size latency  | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_batching_window_p99_9` | Commit batching window size latency  | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_latency_count` | Latency to commit | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_latency_max` | Latency to commit | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_latency_mean` | Latency to commit | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_latency_median` | Latency to commit | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_latency_min` | Latency to commit | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_latency_p25` | Latency to commit | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_latency_p90` | Latency to commit | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_latency_p95` | Latency to commit | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_latency_p99` | Latency to commit | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_commit_latency_p99_9` | Latency to commit | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_data_lag_seconds` | Lag in seconds of the process role | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_durable_bytes_counter` | Storage and Log input rates durable | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_durable_bytes_hz` | Storage and Log input rates durable | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_durable_bytes_roughness` | Storage and Log input rates durable | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_durable_lag_seconds` | Lag in seconds of data being durable of the process role | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_fetched_versions_counter` | Frequency of fetched versions in control plane | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_fetched_versions_hz` | Frequency of fetched versions in control plane | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_fetched_versions_roughness` | Frequency of fetched versions in control plane | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_fetches_from_log_counter` | Frequency of fetched data from T logs | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_fetches_from_log_hz` | Frequency of fetched data from T logs | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_fetches_from_log_roughness` | Frequency of fetched data from T logs | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_finished_queries_counter` | Number of finished queries | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_finished_queries_hz` | Number of finished queries | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_finished_queries_roughness` | Number of finished queries | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_input_bytes_counter` | Storage and Log Input Rates | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_input_bytes_hz` | Storage and Log Input Rates | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_input_bytes_roughness` | Storage and Log Input Rates | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_keys_queried_counter` | Frequency of read storage server operations in bytes | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_keys_queried_hz` | Frequency of read storage server operations in bytes | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_keys_queried_roughness` | Frequency of read storage server operations in bytes | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_kvstore_available_bytes` | KVStore available bytes | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_kvstore_free_bytes` | KVStore free bytes | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_kvstore_used_bytes` | KVStore used bytes | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_low_priority_queries_counter` | Number of low prio queries | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_low_priority_queries_hz` | Number of low prio queries | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_low_priority_queries_roughness` | Number of low prio queries | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_mutation_bytes_counter` | Frequency of mutations in bytes | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_mutation_bytes_hz` | Frequency of mutations in bytes | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_mutation_bytes_roughness` | Frequency of mutations in bytes | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_mutation_counter` | Frequency of mutation | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_mutation_hz` | Frequency of mutation | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_mutation_roughness` | Frequency of mutation | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_queue_disk_available_bytes` | Available bytes in the queue of a process | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_queue_disk_free_bytes` | Free bytes in the queue of a process | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_queue_disk_total_bytes` | Total bytes in the queue of a process | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_queue_disk_used_bytes` | Used bytes in the queue of a process | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_queue_max` | Queue of read queries | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_read_latency_count` | Latency of read | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_read_latency_max` | Latency of read | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_read_latency_mean` | Latency of read | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_read_latency_median` | Latency of read | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_read_latency_min` | Latency of read | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_read_latency_p25` | Latency of read | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_read_latency_p90` | Latency of read | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_read_latency_p95` | Latency of read | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_read_latency_p99` | Latency of read | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_read_latency_p99_9` | Latency of read | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_total_queries_counter` | Total number of queries | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_total_queries_hz` | Total number of queries | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_role_total_queries_roughness` | Total number of queries | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_process_uptime` | Uptime of the process | `["class_type","machine_id","process_id"]` | GAUGE |
| `fdb_cluster_state` | Current state of the cluster (see src/status_models/cluster_data.rs) | `null` | GAUGE |
| `fdb_cluster_total_disk_used_bytes` | Total number of bytes used on all disk | `null` | GAUGE |
| `fdb_cluster_total_kv_size_bytes` | Total number of bytes for all key values | `null` | GAUGE |
| `fdb_database_available` | Database can receive request (0=unavailable) | `null` | GAUGE |
| `fdb_database_healthy` | Database healthiness (0=unhealthy) | `null` | GAUGE |
| `fdb_exporter_parsing_error_count` | Number of parsing errors encountered | `null` | COUNTER |
| `fdb_qos_batch_transactions_per_second_limit` | Number of batch transactions the cluster allows per second | `null` | GAUGE |
| `fdb_qos_limiting_data_lag_storage_server_seconds` | Lag of the limiting storage server (seconds) | `null` | GAUGE |
| `fdb_qos_limiting_data_lag_storage_server_versions` | Lag of the limiting storage server (versions) | `null` | GAUGE |
| `fdb_qos_limiting_durability_lag_storage_server_seconds` | Durability lag of the limiting storage server (seconds) | `null` | GAUGE |
| `fdb_qos_limiting_durability_lag_storage_server_versions` | Durability lag of the limiting storage server (versions) | `null` | GAUGE |
| `fdb_qos_limiting_queue_storage_server_bytes` | Queue of the storage server limiting the system | `null` | GAUGE |
| `fdb_qos_performance_limited_by_reason` | Reason of the system being limited | `null` | GAUGE |
| `fdb_qos_transactions_per_second_limit` | Number of transactions the cluster allows per second | `null` | GAUGE |
| `fdb_qos_worst_data_lag_storage_server_seconds` | Storage server with the worst queue (seconds) | `null` | GAUGE |
| `fdb_qos_worst_data_lag_storage_server_versions` | Storage server with the worst queue (versions) | `null` | GAUGE |
| `fdb_qos_worst_durability_lag_storage_server_seconds` | Storage server with the worst durability queue (seconds) | `null` | GAUGE |
| `fdb_qos_worst_durability_lag_storage_server_versions` | Storage server with the worst durability queue (versions) | `null` | GAUGE |
| `fdb_qos_worst_queue_log_server_bytes` | Worst queue of log server in bytes | `null` | GAUGE |
| `fdb_qos_worst_queue_storage_server_bytes` | Worst queue of storage server | `null` | GAUGE |


## How to generate

1. Run `docker-compose.yml` with `docker compose up -d`
2. Ensure `curl localhost:9090` outputs Metrics
3. Download [`prom2json`](https://github.com/prometheus/prom2json) on your system
4. Ensure you got `jq` on your system
5. Execute jq request below
6. Rewrite the whole table with output data

```
./prom2json http://localhost:9090 | jq 'sort_by(.name) | .[] | @text"| `\(.name)` | \(.help) | `\((.metrics[0]?.labels? | try keys catch null))` | \(.type) |"' -r
```