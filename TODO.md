# TODO

## Bugs

* `Notifier` should maybe call `NotificationHandle.close()` to avoid
  consuming all the notification slots.
* Lots of `unwrap()`s that should return errors.

## Features

* Remote check syncing over gRPC
    - Notify on all synced metrics.
    - Show synced metrics in UI.
    - Show status of syncing in checks
    - Run collector at boot
    - collector build and deploy script
    - Force remote checks and metrics
    - Retrieve remote logs
    - Cache connection and re-use between invocations.
    - Mutual TLS
    - Real-time streaming instead of polling
* Show i64 metric checks in green for Ok or red for Err.
* View old logs
* Push checks (e.g. webhooks for Travis)
    - Travis makes HTTPS request to a listening HTTPS server (outside firewall)
    - HTTPS Server turns that HTTPS request into a message down a
      socket (TCP or WebSocket) to the `monitor` server
    - `monitor` server handler wakes up and pushes the metric into `MetricStore`.
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

## Improvements

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
* Load metrics, checks from a config file, with hot reload. Maybe use
  [RON](https://github.com/ron-rs/ron) or
  [rudano](https://crates.io/crates/rudano)?

## Questions

* `Arc<Mutex<State>>` external or internal?
    - A struct may have better (more concurrent, faster, cheaper) ways
      of implementing internal mutability itself than a Mutex, e.g. a
      concurrent data structure.
    - External wrappers give the user the choice about how to control the struct
    - Internal wrappers are easier to use: consumers can screw up the locking

## `shell_check` timeout notes

* Crate process_control. Terminates the process from another thread,
  doesn't support merging stdout and stderr.
* Various solutions suggested here:
  https://stackoverflow.com/questions/282176/waitpid-equivalent-with-timeout
* Poll for completion?
* sigtimedwait for SIGCHLD, but signals are still nasty and need masking on all threads
  https://man7.org/linux/man-pages/man2/sigtimedwait.2.html
* signalfd for SIGCHLD, but signals are still nasty and need masking on all threads, Linux only
    - See https://ldpreload.com/blog/signalfd-is-useless
* `pidfd_open` (requires Linux 5.3, I'm on 4.15.0)
    - Crate for this: https://docs.rs/mio-pidfd/0.1.1/mio_pidfd/index.html
* wait without timeout and get interrupted by SIGALRM
* kill -9 from another thread on reaching timeout
* https://docs.rs/signal-hook/0.1.16/signal_hook/index.html
* nix crate support for signals:
    - https://docs.rs/nix/0.19.0/nix/sys/signal/index.html
    - https://docs.rs/nix/0.19.0/nix/sys/signalfd/index.html
