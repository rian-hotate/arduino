// app/tasks/ble_handle.rs
use std::sync::mpsc;

use crate::app::ble::ble_command::BleCommand;

#[derive(Clone)]
pub struct BleHandle {
    #[allow(dead_code)]
    pub(crate) tx: mpsc::Sender<BleCommand>,
}

impl BleHandle {
    #[allow(dead_code)]
    pub fn start_pairing_60s(&self) {
        let _ = self
            .tx
            .send(BleCommand::StartAdvertise { timeout_ms: 60000 });
    }

    #[allow(dead_code)]
    pub fn stop_pairing(&self) {
        let _ = self.tx.send(BleCommand::StopAdvertise);
    }
}
