#[derive(Clone, Debug)]
pub enum BleCommand {
    StartAdvertise { timeout_ms: u32 },
    StopAdvertise,
    DisconnectAll, // 全クライアント切断
    GetStatus,     // ステータス取得
    Shutdown,      // タスク終了
}
