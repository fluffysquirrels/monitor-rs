# TODO

## Bugs

* `Notifier` should call `NotificationHandle.close()` to avoid
  consuming all the notification slots.

## Features

* Other metric types than `OkErr`.
* View just failing checks.
* Web front end. [actix-web](https://github.com/actix/actix-web) looks good
* Distributed architecture. [sqlx](https://github.com/launchbadge/sqlx) looks good for DB access,
  [rsedis](https://github.com/seppo0010/rsedis) could be nice for real-time pubsub.
* Web connectivity check, probably using one of:
    - http://connectivitycheck.gstatic.com/generate_204
    - http://www.msftconnecttest.com/connecttest.txt

## Improvements

* `MetricStoreDataSource` should use time for `Point.t`.
* `MetricStoreDataSource` Only return points when they're new.
* High CPU usage when not minimised (reduce FPS, show fewer graphs?)
* No way to terminate a Scheduler
* No visual feedback from using "Force" button. It should be disabled
  when you click it and enabled again when the job finishes.
* No way to tell hovering over the check label will show you the log.
* `Scheduler` could use a heap to calculate the next jobs in
  `O(log n)` time each, rather than iterating through all jobs frequently in
  `O(n)` time.
* `Scheduler` should wait on a channel instead of sleeping unconditionally.
  Then it can respond quickly to signals to shut down or start a forced job.
* Load metrics, checks from a config file, with hot reload. Maybe use
  [RON](https://github.com/ron-rs/ron) or
  [rudano](https://crates.io/crates/rudano)?

## Questions

* `Arc<Mutex<State>>` external or internal?
    - A struct may have better (more concurrent, faster, cheaper) ways
      of implementing internal mutability itself than a Mutex, e.g. a
      concurrent data structure.
    - External wrappers give the user the choice about how to control the struct
    - Internal wrappers are probably easier to use

## `shell_check` timeout notes

* Various solutions suggested here:
  https://stackoverflow.com/questions/282176/waitpid-equivalent-with-timeout
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
