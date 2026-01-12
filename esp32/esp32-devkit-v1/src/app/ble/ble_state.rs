#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BleState {
    Idle = 0,
    Advertising = 1,
    Connected = 2,
    Disconnected = 3,
    Error = 255,
}

impl From<u8> for BleState {
    fn from(v: u8) -> Self {
        match v {
            1 => BleState::Advertising,
            2 => BleState::Connected,
            3 => BleState::Disconnected,
            255 => BleState::Error,
            _ => BleState::Idle,
        }
    }
}
