# TODO

## Bugs

* `shell_check` has no timeout
* `connect: Network is unreachable` in console from `ping.mf` when wi-fi disconnected.
* `MetricStoreDataSource` should use time for `Point.t`.
* `MetricStoreDataSource` Only return points when they're new.
* High CPU usage when not minimised (reduce FPS, show fewer graphs?)
* `Notifier` should call `NotificationHandle.close()` to avoid
  consuming all the notification slots.
* Upload the repo somewhere as a backup.

## Features

* Other metric types than `OkErr`.
* View just failing checks.
* Web front end
* Web connectivity check, probably using one of:
  - http://connectivitycheck.gstatic.com/generate_204
  - http://www.msftconnecttest.com/connecttest.txt

## Improvements

* No way to terminate a Scheduler
* No visual feedback from using "Force" button.
* `Scheduler` could use a heap to calculate the next jobs in
  `O(log n)` time each, rather than iterating through all jobs frequently in
  `O(n)` time.
* Load metrics, checks from a config file, with hot reload. Maybe use RON or rudano?

## Questions

* `Arc<Mutex<State>>` external or internal.
