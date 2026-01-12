#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BleEvent {
    StartAdvertise,
    StopAdvertise,
    Connected,
    Disconnected,
    Error,
}
