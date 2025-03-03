use crate::models::packet::NtpPacket;
use crate::models::ntp_flags::{NtpModeFlags, NtpLeapFlags};
use crate::utils;

pub struct NtpResolver {

}

impl NtpResolver {
    pub fn resolve(packet: &NtpPacket) -> NtpPacket {
        let mut cloned: NtpPacket = packet.clone();

        cloned.set_mode_flag(NtpModeFlags::Server);
        cloned.set_leap_flag(NtpLeapFlags::NoWarning);
        cloned.stratum = 1;
        cloned.poll = 6;
        cloned.precision = -20;
        cloned.root_delay = 0x00010000;
        cloned.root_dispersion = 0x00010000;

        cloned.ref_id = u32::from_be_bytes(*b"ATOM");
        cloned.recv_timestamp = utils::get_ntp_timestamp();
        cloned.origin_timestamp = utils::get_ntp_timestamp();
        cloned.ref_timestamp = utils::get_ntp_timestamp();

        cloned
    }
}
