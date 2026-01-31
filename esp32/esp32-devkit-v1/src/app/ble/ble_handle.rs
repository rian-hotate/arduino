// app/tasks/ble_handle.rs
use std::sync::mpsc;

use crate::app::ble::ble_command::BleCommand;

#[derive(Clone)]
pub struct BleHandle {
    pub(crate) tx: mpsc::Sender<BleCommand>,
}
