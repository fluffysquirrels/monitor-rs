# TODO

## WIP

* Web front-end
    * Get metric updates over WebSocket
        * Fix update race because WebSocket doesn't return initial values.

## Bugs

* `Notifier` should maybe call `NotificationHandle.close()` to avoid
  consuming all the notification slots.
* Fix update race because WebSocket doesn't return initial values.

## Features

* Web front-end
    * WebSocket
        * Ping frames from client
    * Notifications when page is open
    * Notifications when page is not open?
    * Authentication
        * Track login time?
        * Test expired sessions.
        * Session storage with in-memory backing for now (move to DB
          or out of process in-memory store later)
            * This is a big old memory leak right now. Enforce one session per user?
        * Refresh sessions near their expiry? (50% towards expiry?)
    * Anti-CSRF
    * Show check results
        * Start with statically rendered HTML
        * Then JS rendered from JSON injected into the page
        * Then JS rendered from JSON sent over a WebSocket
    * Notifications
* Show count of checks that are not yet set (no .latest) in summary line?
* View checks filtered by containing a string.
* Add hyperlinks to show logs / status. E.g. Travis checks, jellyfin
* Push checks (e.g. webhooks for Travis)
    * Travis makes HTTPS request to a listening HTTPS server (outside firewall)
    * HTTPS Server turns that HTTPS request into a gRPC RPC or gRPC
      stream message to the `collector` server
    * `collector` server handler wakes up and pushes the metric into `MetricStore`.
* Configure flakey metric detection: need at least n failures before notification.
* Metric type F64.
* DB:
  [sled](https://github.com/spacejam/sled) or
  postgres.
* Distributed architecture libs:
  [tonic](https://github.com/hyperium/tonic) looks good for gRPC,
  [sqlx](https://github.com/launchbadge/sqlx) looks good for DB access,
  [rsedis](https://github.com/seppo0010/rsedis) and
  [redis-rs](https://github.com/mitsuhiko/redis-rs) could be nice for real-time pubsub.
* Keep a history of checks and logs in a DB. View old logs in UI.
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

* monitor gtk-client: Random exponential backoff for connection retries.
* monitor web: Random exponential backoff for connection retries.
* Lots of `unwrap()`s, `expect()`s that should return errors, use clippy.
* Sync jobs fail loudly on resume, would be nice to special case this
  like the scheduler sleep on resume.
* Separate CA's for clients and servers; currently server certs can be used as sync client certs.
* Ideally it would be zero lines of config to show a remote check in the UI.
* The scheduler detects resume from suspend, it would be nice to restart
  the sync jobs at the same time.
* Force run remote job should probably re-use an existing tokio runtime.
* Force run remote job should probably retry a few times before failing.
* Remote sync config should be in an Arc, it's expensive and it's copied a few times.
* Measure latency between a forced check request and its metric and log being published,
  including for remote checks.
* Lots of duplication between remote syncing logs and metrics. Revisit a `table` abstraction.
* Consider separate Cargo.tomls for client and collector, so collector can build
  without gtk.
* More of a visual separator in the GUI between metrics to help show
  which check the buttons belong to.
* `MetricStoreDataSource` should use time for `Point.t`.
* No visual feedback from using "Force" button. It should be disabled
  when you click it and enabled again when the job finishes.
* Force button fails for sync checks, because they don't use the Scheduler.
* No way to tell hovering over the check label will show you the log.
* `Scheduler` could use a heap to calculate the next jobs in
  `O(log n)` time each, rather than iterating through all jobs frequently in
  `O(n)` time.
* `Scheduler` can sleep more intelligently: until the next job is due.
* Load testing.
  [ghz](https://github.com/bojand/ghz) was good for request-response
  rpc's, didn't seem to work for streaming
* Remote check syncing over gRPC
    * What happens when the collector or client is overloaded? How would we shed load?
        * Hopefully the `tokio::mpsc` the streaming rpc uses will back up,
          and `try_send` will fail. Could test this by not reading from the client side.

## Questions

* `Arc<Mutex<State>>` external or internal?
    * A struct may have better (more concurrent, faster, cheaper) ways
      of implementing internal mutability itself than a Mutex, e.g. a
      concurrent data structure.
    * External wrappers give the user the choice about how to control the struct
    * Internal wrappers are easier to use: consumers can screw up the locking
