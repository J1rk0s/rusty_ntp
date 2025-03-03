#[repr(u8)]
pub enum NtpModeFlags {
    SymmetricActive = 1,
    SymmetricPasive = 2,
    Client = 3,
    Server = 4,
    Broadcast = 5,
    Control = 6
}

#[repr(u8)]
pub enum NtpLeapFlags {
    NoWarning = 0,
    LastMinuteSO = 1,
    LastMinuteFN = 2,
    Unknown = 3
}