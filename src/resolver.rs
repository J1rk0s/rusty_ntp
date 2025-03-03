use std::time::SystemTime;

use crate::models::packet::NtpPacket;
use crate::models::ntp_flags::NtpModeFlags;

pub struct NtpResolver {

}

impl NtpResolver {
    pub fn resolve(packet: &NtpPacket) -> NtpPacket {
        let mut cloned: NtpPacket = packet.clone();

        cloned.set_mode_flag(NtpModeFlags::Server);
        cloned.stratum = 1;
        cloned.poll = 6;
        cloned.precision = -20;
        cloned.root_delay = 0x00010000;
        cloned.root_dispersion = 0x00010000;

        cloned.ref_id = u32::from_le_bytes(*b"ATOM");
    

        cloned
    }
}
