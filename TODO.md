# TODO

## Bugs

* `shell_check` has no timeout
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
