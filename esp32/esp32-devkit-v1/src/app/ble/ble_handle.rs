// app/tasks/ble_handle.rs
use std::sync::mpsc;

use crate::app::ble::ble_command::BleCommand;

#[derive(Clone)]
pub struct BleHandle {
    pub(crate) tx: mpsc::Sender<BleCommand>,
}

impl BleHandle {
    pub fn start_pairing_60s(&self) {
        let _ = self
            .tx
            .send(BleCommand::StartAdvertise { timeout_ms: 60000 });
    }

    pub fn stop_pairing(&self) {
        let _ = self.tx.send(BleCommand::StopAdvertise);
    }
}
