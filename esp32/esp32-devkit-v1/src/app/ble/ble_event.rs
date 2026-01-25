/// BLEタスクから発行される状態変化イベント
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BleEvent {
    /// アドバタイズ開始
    AdvertisingStarted,
    /// アドバタイズ停止
    AdvertisingStopped,
    /// デバイス接続
    #[allow(dead_code)]
    Connected,
    /// デバイス切断
    #[allow(dead_code)]
    Disconnected,
    /// エラー発生
    #[allow(dead_code)]
    Error,
}
