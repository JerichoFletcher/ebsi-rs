use std::time::Duration;

use serenity::model::Timestamp;

pub fn timestamp_to_string(t: &Timestamp) -> String {
    let date = t.date();
    let time = t.time();
    format!("{} {} {}, {:0>2}:{:0>2}:{:0>2} (GMT)", time.hour(), time.minute(), time.second(), date.day(), date.month(), date.year())
}

pub fn duration_to_string(d: &Duration) -> String {
    let mut str = String::new();

    let raw = d.as_secs();
    let days = raw / (60 * 60 * 24);
    let hours = (raw % (60 * 60 * 24)) / (60 * 60);
    let minutes = (raw % (60 * 60)) / 60;
    let seconds = raw % 60;

    if days > 0 { str.push_str(format!("{days} hari ").as_str()); }
    if hours > 0 { str.push_str(format!("{hours} jam ").as_str()); }
    if minutes > 0 { str.push_str(format!("{minutes} menit ").as_str()); }
    if days == 0 && hours == 0 { str.push_str(format!("{seconds} detik").as_str()); }

    str
}
