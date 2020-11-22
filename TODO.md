# TODO

## WIP

## Bugs

* `Notifier` should maybe call `NotificationHandle.close()` to avoid
  consuming all the notification slots.
* Lots of `unwrap()`s, `expect()`s that should return errors, use clippy.

## Features

* Remote check syncing over gRPC
    * Show status of syncing in checks
    * Migrate more mf jobs to collector
    * Notify on all synced metrics.
        * Including I64 metrics.
    * Show synced metrics in UI.
        * Ideally zero lines of config in client for synced metrics (incl checks on metrics?)
    * What happens when the collector or client is overloaded? How would we shed load?
        * Hopefully the `tokio::mpsc` the streaming rpc uses will back up,
          and `try_send` will fail. Could test this by not reading from the client side.
* Show i64 metric checks in green for Ok or red for Err.
* View old logs
* Push checks (e.g. webhooks for Travis)
    * Travis makes HTTPS request to a listening HTTPS server (outside firewall)
    * HTTPS Server turns that HTTPS request into a gRPC RPC or gRPC
      stream message to the `collector` server
    * `collector` server handler wakes up and pushes the metric into `MetricStore`.
* Configure flakey metric detection: need at least n failures before notification.
* Metric type F64.
* View just failing checks.
* Scroll through list of checks when they get too long.
* Web or Android front end. [actix-web](https://github.com/actix/actix-web) looks good
* Distributed architecture libs:
  [tonic](https://github.com/hyperium/tonic) looks good for gRPC,
  [sqlx](https://github.com/launchbadge/sqlx) looks good for DB access,
  [rsedis](https://github.com/seppo0010/rsedis) and
  [redis-rs](https://github.com/mitsuhiko/redis-rs) could be nice for real-time pubsub.
* Keep a history of checks and logs in a DB.
* CI
    * Triggered by a webhook / rpc / force run for a manual check.
    * Stream job stdout log as job progresses
        * Await chunks of bytes from sub-process
          (see [tokio::process](https://docs.rs/tokio/0.2.22/tokio/process/index.html)),
          mark them with chrono::Utc time, broadcast them in a signal from LogStoreV2,
          LogStoreV2 saves them to a DB or file,
          collector streams them over gRPC to client,
          client LogStoreV2 keeps them in memory only long enough to render to the GUI.
* Log shipping. In a separate tool?
* Streaming checks / metrics from a single execution
    * Push to a monitor hub over gRPC?
    * New-line framed JSON or COBS framed protobuf over stdout?

## Improvements

* The scheduler detects resume from suspend, it would be nice to force
  the sync jobs at the same time.
* Force run remote job should probably re-use an existing tokio runtime.
* Force run remote job should probably retry a few times before failing.
* apt.upgradable checks should be metrics to show the number of packages upgradable in the UI.
* Remote sync config should be in an Arc, it's expensive and it's copied a few times.
* Measure latency between a forced check request and its metric and log being published,
  including for remote checks.
* Lots of duplication between remote syncing logs and metrics. Revisit a `table` abstraction.
* Duplication in remote syncing in error cases: handle at top of loop or in wrapper function?
* Consider separate Cargo.tomls for client and collector, so collector can build
  without gtk.
* More of a visual separator in the GUI between metrics to help show
  which check the buttons belong to.
* `MetricStoreDataSource` should use time for `Point.t`.
* No visual feedback from using "Force" button. It should be disabled
  when you click it and enabled again when the job finishes.
* No way to tell hovering over the check label will show you the log.
* `Scheduler` could use a heap to calculate the next jobs in
  `O(log n)` time each, rather than iterating through all jobs frequently in
  `O(n)` time.
* `Scheduler` can sleep more intelligently: until the next job is due.
* Load testing.
  [ghz](https://github.com/bojand/ghz) was good for request-response
  rpc's, didn't seem to work for streaming

## Questions

* `Arc<Mutex<State>>` external or internal?
    * A struct may have better (more concurrent, faster, cheaper) ways
      of implementing internal mutability itself than a Mutex, e.g. a
      concurrent data structure.
    * External wrappers give the user the choice about how to control the struct
    * Internal wrappers are easier to use: consumers can screw up the locking

## `shell_check` timeout notes

* Crate process_control. Terminates the process from another thread,
  doesn't support merging stdout and stderr.
* Various solutions suggested here:
  https://stackoverflow.com/questions/282176/waitpid-equivalent-with-timeout
* Poll for completion?
* sigtimedwait for SIGCHLD, but signals are still nasty and need masking on all threads
  https://man7.org/linux/man-pages/man2/sigtimedwait.2.html
* signalfd for SIGCHLD, but signals are still nasty and need masking on all threads, Linux only
    * See https://ldpreload.com/blog/signalfd-is-useless
* `pidfd_open` (requires Linux 5.3, I'm on 4.15.0)
    * Crate for this: https://docs.rs/mio-pidfd/0.1.1/mio_pidfd/index.html
* wait without timeout and get interrupted by SIGALRM
* kill -9 from another thread on reaching timeout
* https://docs.rs/signal-hook/0.1.16/signal_hook/index.html
* nix crate support for signals:
    * https://docs.rs/nix/0.19.0/nix/sys/signal/index.html
    * https://docs.rs/nix/0.19.0/nix/sys/signalfd/index.html

## Deploy new collector notes

```
sudo adduser --system --disabled-password --no-create-home monitor-collector
sudo mkdir -p /usr/local/lib/monitor
sudo setfacl -m user:alex:rwx /usr/local/lib/monitor
```

Copy systemd service `${REPO}/conf/monitor-collector.service` to
target at `/usr/local/lib/systemd/system/monitor-collector.service`
make sure it is owned by root and not world writable
```
sudo systemctl daemon-reload
sudo systemctl enable monitor-collector.service
sudo systemctl restart monitor-collector.service
sleep 1
sudo systemctl status monitor-collector.service
```

Put this in sudoers, replacing `$(hostname)`
```
# Allow alex to run, stop, or restart the monitor-collector service
alex $(hostname)=(root) NOPASSWD: /bin/systemctl restart monitor-collector.service, /bin/systemctl stop monitor-collector.service, /bin/systemctl start monitor-collector.service
```

Copy the cert and key to the server, set key permissions to read only by monitor-collector.

Now use `${REPO}/bin/deploy_collectors`
