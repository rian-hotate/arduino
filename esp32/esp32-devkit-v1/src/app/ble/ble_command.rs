#[derive(Clone, Debug)]
pub enum BleCommand {
    StartAdvertise {
        timeout_ms: u32,
    },
    StopAdvertise,
    /// 現在のBLE接続状態を取得
    GetState,
    Shutdown,
}
