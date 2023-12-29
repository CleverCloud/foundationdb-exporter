use crate::{metrics::MetricsConvertible, status_models::cluster_process_disk::ClusterProcessDisk};
use lazy_static::lazy_static;
use prometheus::{register_gauge_vec, register_int_gauge_vec, GaugeVec, IntGaugeVec};

lazy_static! {
    static ref P_PROCESS_DISK_BUSY: GaugeVec = register_gauge_vec!(
        "fdb_cluster_process_disk_busy",
        "Disk is being busy (0.0 to 1.0 value)",
        &["machine_id", "process_id", "class_type"]
    )
    .unwrap();
    static ref P_PROCESS_DISK_FREE_BYTES: IntGaugeVec = register_int_gauge_vec!(
        "fdb_cluster_process_disk_free_bytes",
        "Bytes available on the disk used by process",
        &["machine_id", "process_id", "class_type"]
    )
    .unwrap();
    static ref P_PROCESS_DISK_TOTAL_BYTES: IntGaugeVec = register_int_gauge_vec!(
        "fdb_cluster_process_disk_total_bytes",
        "Bytes total on the disk used by process",
        &["machine_id", "process_id", "class_type"]
    )
    .unwrap();
    static ref P_PROCESS_DISK_READS_COUNTER: IntGaugeVec = register_int_gauge_vec!(
        "fdb_cluster_process_disk_reads_count",
        "Number of reads on the disk",
        &["machine_id", "process_id", "class_type"],
    )
    .unwrap();
    static ref P_PROCESS_DISK_READS_FREQ: GaugeVec = register_gauge_vec!(
        "fdb_cluster_process_disk_reads_frequency",
        "Frequency of reads on the disk",
        &["machine_id", "process_id", "class_type"],
    )
    .unwrap();
    static ref P_PROCESS_DISK_READS_SECTORS: GaugeVec = register_gauge_vec!(
        "fdb_cluster_process_disk_reads_sectors",
        "N/A",
        &["machine_id", "process_id", "class_type"],
    )
    .unwrap();
    static ref P_PROCESS_DISK_WRITES_COUNTER: IntGaugeVec = register_int_gauge_vec!(
        "fdb_cluster_process_disk_writes_count",
        "Number of writes on the disk",
        &["machine_id", "process_id", "class_type"],
    )
    .unwrap();
    static ref P_PROCESS_DISK_WRITES_FREQ: GaugeVec = register_gauge_vec!(
        "fdb_cluster_process_disk_writes_frequency",
        "Frequency of writes on the disk",
        &["machine_id", "process_id", "class_type"],
    )
    .unwrap();
    static ref P_PROCESS_DISK_WRITES_SECTORS: GaugeVec = register_gauge_vec!(
        "fdb_cluster_process_disk_writes_sectors",
        "N/A",
        &["machine_id", "process_id", "class_type"],
    )
    .unwrap();
}

impl MetricsConvertible for ClusterProcessDisk {
    fn to_metrics(&self, labels: &[&str]) {
        P_PROCESS_DISK_BUSY.with_label_values(labels).set(self.busy);
        P_PROCESS_DISK_FREE_BYTES
            .with_label_values(labels)
            .set(self.free_bytes);
        P_PROCESS_DISK_TOTAL_BYTES
            .with_label_values(labels)
            .set(self.total_bytes);

        P_PROCESS_DISK_READS_FREQ
            .with_label_values(labels)
            .set(self.reads.hz);
        P_PROCESS_DISK_READS_COUNTER
            .with_label_values(labels)
            .set(self.reads.counter);
        P_PROCESS_DISK_READS_SECTORS
            .with_label_values(labels)
            .set(self.reads.sectors);

        P_PROCESS_DISK_WRITES_FREQ
            .with_label_values(labels)
            .set(self.writes.hz);
        P_PROCESS_DISK_WRITES_COUNTER
            .with_label_values(labels)
            .set(self.writes.counter);
        P_PROCESS_DISK_WRITES_SECTORS
            .with_label_values(labels)
            .set(self.writes.sectors);
    }
}
