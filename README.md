# FoundationDB Metrics Exporter

A tool which will poll status of your FoundationDB cluster and expose human-readable
metrics for Prometheus. When it is useful, metrics are tagged with appropriate data
to be easily retriveable.

Metrics this exporter exposes are available in **[METRICS.md](./METRICS.md)**.

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
  -d, --delay-sec <DELAY_SEC>  Delay in seconds between two update of the status & metrics [env: FDB_EXPORTER_DELAY=] [default: 15]
  -h, --help                   Print help
  -V, --version                Print version
```

## Build

Rust `1.74.0` at least is required

```
cargo build --release
./target/release/foundationdb-exporter
```

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md)

