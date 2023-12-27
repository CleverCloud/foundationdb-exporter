# FoundationDB Metrics Exporter

A tool which will poll status of your FoundationDB cluster and expose human-readable
metrics for Prometheus.

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
