use crate::OkErr;

pub struct Notifier {
    metrics: Vec<NotifierMetric>,
}

struct NotifierMetric {
    name: String,
    last_value: OkErr,
    last_notification: Option<chrono::DateTime<chrono::Utc>>,
}

const NOTIFICATION_REFRESH_SECS: i64 = 5 * 60; // 5 minutes

impl Notifier {
    pub fn new() -> Notifier {
        Notifier {
            metrics: vec![],
        }
    }

    pub fn update_metric(&mut self, name: &str, new_value: OkErr) {
        let existing = self.metrics.iter_mut().find(|m| m.name == name);
        let metric: &mut NotifierMetric = match existing {
            Some(m) => m,
            None => {
                self.metrics.push(NotifierMetric {
                    name: name.to_owned(),
                    last_value: OkErr::Ok,
                    last_notification: None,
                });
                self.metrics.last_mut().unwrap()
            }
        };
        let last_value = metric.last_value;
        metric.last_value = new_value;

        let is_changed = last_value != new_value;
        let is_old = metric.last_notification.is_none()
            || ((chrono::Utc::now() - metric.last_notification.unwrap()) >
                chrono::Duration::seconds(NOTIFICATION_REFRESH_SECS));
        let is_old_error = (last_value == OkErr::Err) && is_old;
        if is_changed || is_old_error {
            trace!("Notifier: is_changed={} is_old_error={}", is_changed, is_old_error);
            let res = notify_rust::Notification::new()
                .summary("monitor")
                .body(&format!("metric `{}` is {:?}", metric.name, metric.last_value))
                .timeout(notify_rust::Timeout::Milliseconds(2000))
                .show();
            metric.last_notification = Some(chrono::Utc::now());
            if let Err(e) = res {
                error!("Showing notification: {}", e);
            }

            // TODO: Close after n seconds. NB: NotificationHandle is !Send.

            // std::thread::sleep(std::time::Duration::from_secs(5));
            // nh.close();
        }
    }
}
