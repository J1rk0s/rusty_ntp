use std::time::{SystemTime, UNIX_EPOCH};

const NTP_UNIX_OFFSET: u64 = 2_208_988_800;

pub fn get_ntp_timestamp() -> u64 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Error getting the time");

    let ntp_seconds = NTP_UNIX_OFFSET + now.as_secs();
    let ntp_fraction = ((now.subsec_nanos() as u64 * (1 << 32)) / 1_000_000_000) as u32;

    (ntp_seconds << 32) | ntp_fraction as u64
}