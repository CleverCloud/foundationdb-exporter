# FoundationDB Metrics Exporter

A tool which will poll status of your FoundationDB cluster and expose human-readable
metrics for Prometheus. When it is useful, metrics are tagged with appropriate data
to be easily retriveable.

*Not all metrics from status are yet available, but the ones we use are available.
If you need more metrics, feel free to contribute!*

## Getting started

```
# Run a FoundationDB cluster with the exporter
docker compose up -d
# Fetch metrics available from the exporter
curl localhost:9090
```

## Help

```
A monitoring tool for FoundationDB with exporting capabilities for prometheus

Usage: foundationdb-exporter [OPTIONS]

Options:
  -p, --port <PORT>            Listening port of the web server [env: FDB_EXPORTER_PORT=] [default: 9090]
  -c, --cluster <CLUSTER>      Location of fdb.cluster file [env: FDB_CLUSTER_FILE=]
  -d, --delay-sec <DELAY_SEC>  Delay in seconds between two update of the status & metrics [env: FDB_EXPORTER_DELAY=] [default: 5]
  -h, --help                   Print help
  -V, --version                Print version
```

## Build

Rust `1.74.0` at least is required

```
cargo build --release
./target/release/foundationdb-exporter
```

## Contributing a new metric

We want to have a symetric structure between `status_models` and `prometheus` exporter
files. A trait is available for new structs: `MetricsConvertible`. When using it,
you should ensure its method `to_metric()` is called by its upper struct in models.

1. Ensure the metric is not yet available by exploring `src/metrics/prometheus/`
2. Check the status key is available in models `src/status_models`, if not, add
   necessary structs for it
3. Implemenent `MetricsConvertible` (`src/metrics/mod.rs`) on the new struct, or
   update existin.
4. Ensure `to_metrics()` method is called on your new implementation
