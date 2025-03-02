#[derive(Debug)]
pub struct NtpPacket {
    pub flags: u8,
    pub stratum: u8,
    pub poll: u8,
    pub precision: u8,
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
        let flags = bytes.get(0)?;
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
            precision: *precision,
            root_delay,
            root_dispersion,
            ref_id,
            ref_timestamp,
            origin_timestamp,
            recv_timestamp,
            trans_timestamp
        })
    }
}