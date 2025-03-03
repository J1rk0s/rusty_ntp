use super::ntp_flags::NtpModeFlags;

#[derive(Debug, Clone)]
pub struct NtpPacket {
    pub flags: u8,
    pub stratum: u8,
    pub poll: u8,
    pub precision: i8,
    pub root_delay: u32,
    pub root_dispersion: u32,
    pub ref_id: u32,
    pub ref_timestamp: u64,
    pub origin_timestamp: u64,
    pub recv_timestamp: u64,
    pub trans_timestamp: u64,
}

impl NtpPacket {
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let flags = bytes.first()?;
        let stratum = bytes.get(1)?;
        let poll = bytes.get(2)?;
        let precision = bytes.get(3)?;

        let root_delay = u32::from_be_bytes(bytes.get(4..8)?.try_into().unwrap());
        let root_dispersion = u32::from_be_bytes(bytes.get(8..12)?.try_into().unwrap());
        let ref_id = u32::from_be_bytes(bytes.get(12..16)?.try_into().unwrap());
        let ref_timestamp = u64::from_be_bytes(bytes.get(16..24)?.try_into().unwrap());
        let origin_timestamp = u64::from_be_bytes(bytes.get(24..32)?.try_into().unwrap());
        let recv_timestamp = u64::from_be_bytes(bytes.get(32..40)?.try_into().unwrap());
        let trans_timestamp = u64::from_be_bytes(bytes.get(40..48)?.try_into().unwrap());

        Some(Self {
            flags: *flags,
            stratum: *stratum,
            poll: *poll,
            precision: *precision as i8,
            root_delay,
            root_dispersion,
            ref_id,
            ref_timestamp,
            origin_timestamp,
            recv_timestamp,
            trans_timestamp
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut res: Vec<u8> = vec![];

        res.push(self.flags);
        res.push(self.stratum);
        res.push(self.poll);
        res.push(self.precision as u8);

        res.extend_from_slice(&self.root_delay.to_be_bytes());
        res.extend_from_slice(&self.root_dispersion.to_be_bytes());
        res.extend_from_slice(&self.ref_id.to_be_bytes());
        res.extend_from_slice(&self.ref_timestamp.to_be_bytes());
        res.extend_from_slice(&self.origin_timestamp.to_be_bytes());
        res.extend_from_slice(&self.recv_timestamp.to_be_bytes());
        res.extend_from_slice(&self.trans_timestamp.to_be_bytes());

        res
    }

    pub fn set_mode_flag(&mut self, flag: NtpModeFlags) -> &mut Self {
        match flag {
            NtpModeFlags::SymmetricActive => {
                self.flags |= flag as u8;
            }

            NtpModeFlags::SymmetricPasive => {
                self.flags |= flag as u8;
            }

            NtpModeFlags::Client => {
                self.flags |= flag as u8;
            }

            NtpModeFlags::Server => {
                self.flags |= flag as u8;
            }

            NtpModeFlags::Control => {
                self.flags |= flag as u8;
            }

            NtpModeFlags::Broadcast => {
                self.flags |= flag as u8;
            }
        }

        self
    }
}
