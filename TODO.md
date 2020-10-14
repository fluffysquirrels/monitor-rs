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
* Way to view a shell check log.
* View just failing checks.
* Web front end

## Improvements

* No way to terminate a Scheduler
* Scheduler shouldn't lock so much / so long
* `Scheduler`, `Notifier` should store data in a BTreeMap.
* No feedback from "Force" button.

## Questions

* `Arc<Mutex<State>>` external or internal.
