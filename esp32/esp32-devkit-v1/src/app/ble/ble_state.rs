/// BLE接続/アドバタイズ状態
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BleState {
    pub connected: bool,
    pub advertising: bool,
    pub error: bool,
}
