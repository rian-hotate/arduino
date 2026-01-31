// app/tasks/ble_handle.rs
use std::sync::mpsc;

use crate::app::ble::ble_command::BleCommand;
use crate::common::Result;

#[derive(Clone)]
pub struct BleHandle {
    pub(crate) tx: mpsc::Sender<BleCommand>,
}

impl BleHandle {
    /// アドバタイズを開始する
    pub fn start_advertising(&self, timeout_ms: u32) -> Result<()> {
        log::info!("BLE: Requesting advertise start (timeout: {}ms)", timeout_ms);
        self.tx
            .send(BleCommand::StartAdvertise { timeout_ms })
            .map_err(|e| {
                log::error!("Failed to send StartAdvertise: {}", e);
                crate::common::Error::new_unexpected(&format!("send command failed: {e}"))
            })
    }

    /// アドバタイズを停止する
    pub fn stop_advertising(&self) -> Result<()> {
        log::info!("BLE: Requesting advertise stop");
        self.tx.send(BleCommand::StopAdvertise).map_err(|e| {
            log::error!("Failed to send StopAdvertise: {}", e);
            crate::common::Error::new_unexpected(&format!("send command failed: {e}"))
        })
    }

    /// BLEタスクをシャットダウンする
    pub fn shutdown(&self) -> Result<()> {
        log::info!("BLE: Requesting shutdown");
        self.tx.send(BleCommand::Shutdown).map_err(|e| {
            log::error!("Failed to send Shutdown: {}", e);
            crate::common::Error::new_unexpected(&format!("send command failed: {e}"))
        })
    }
}
