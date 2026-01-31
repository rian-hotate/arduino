/// BLEタスクから発行される状態変化イベント
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BleEvent {
    /// アドバタイズ開始
    AdvertisingStarted,
    /// アドバタイズ停止
    AdvertisingStopped,
    /// デバイス接続
    Connected,
    /// デバイス切断
    Disconnected,
    /// エラー発生
    Error,
    /// 接続状態応答 (true = 接続中, false = 未接続)
    StateResponse(bool),
}
