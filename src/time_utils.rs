use chrono::TimeZone;
use crate::monitor_core_types;

pub fn chrono_datetime_from_protobuf(t: &monitor_core_types::Time
) -> Result<chrono::DateTime<chrono::Utc>, String> {
    let epoch = chrono::Utc.ymd(1970, 1, 1).and_hms(0, 0, 0);
    Ok(epoch
       + chrono::Duration::milliseconds(t.epoch_millis)
       + chrono::Duration::nanoseconds(t.nanos as i64)
    )
}

pub fn chrono_datetime_to_protobuf(t: &chrono::DateTime<chrono::Utc>
) -> Result<monitor_core_types::Time, String> {
    Ok(monitor_core_types::Time {
        epoch_millis: t.timestamp_millis(),
        nanos: t.timestamp_subsec_nanos() % 1_000_000,
    })
}

pub fn std_time_duration_from_protobuf(d: &monitor_core_types::Duration
) -> Result<std::time::Duration, String> {
    Ok(std::time::Duration::new(d.secs, d.nanos))
}

pub fn std_time_duration_to_protobuf(d: &std::time::Duration
) -> Result<monitor_core_types::Duration, String> {
    Ok(monitor_core_types::Duration {
        secs: d.as_secs(),
        nanos: d.subsec_nanos(),
    })
}
