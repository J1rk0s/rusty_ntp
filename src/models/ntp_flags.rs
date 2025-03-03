#[repr(u8)]
pub enum NtpModeFlags {
    SymmetricActive = 1,
    SymmetricPasive = 2,
    Client = 3,
    Server = 4,
    Broadcast = 5,
    Control = 6
}