pub mod event_coordinator;
pub mod task_manager;

use std::sync::{mpsc, Arc, Mutex};

use crate::app::ble::{ble_event::BleEvent, ble_handle::BleHandle};
use crate::app::button::event::ButtonEvent;
use crate::app::led::led_handle::LedHandle;

pub use task_manager::TaskManager;

/// タスク間共有の状態を持つ構造体（各タスクのハンドルを保持）
pub struct Tasks {
    led_handle: Mutex<Option<LedHandle>>,
    ble_handle: Mutex<Option<BleHandle>>,
    button_event_tx: Mutex<Option<mpsc::Sender<ButtonEvent>>>,
    ble_event_tx: Mutex<Option<mpsc::Sender<BleEvent>>>,
}

impl Tasks {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            led_handle: Mutex::new(None),
            ble_handle: Mutex::new(None),
            button_event_tx: Mutex::new(None),
            ble_event_tx: Mutex::new(None),
        })
    }

    pub fn set_led_handle(&self, handle: LedHandle) {
        *self.led_handle.lock().unwrap() = Some(handle);
    }

    pub fn set_ble_handle(&self, handle: BleHandle) {
        *self.ble_handle.lock().unwrap() = Some(handle);
    }

    pub fn send_led_command(&self, cmd: crate::app::led::led_command::LedCommand) {
        if let Some(handle) = self.led_handle.lock().unwrap().as_ref() {
            let _ = handle.tx.send(cmd);
        }
    }

    #[allow(dead_code)]
    pub fn send_ble_command(&self, cmd: crate::app::ble::ble_command::BleCommand) {
        if let Some(handle) = self.ble_handle.lock().unwrap().as_ref() {
            let _ = handle.tx.send(cmd);
        }
    }

    pub fn set_button_event_tx(&self, tx: mpsc::Sender<ButtonEvent>) {
        *self.button_event_tx.lock().unwrap() = Some(tx);
    }

    pub fn send_button_event(&self, event: ButtonEvent) {
        if let Some(tx) = self.button_event_tx.lock().unwrap().as_ref() {
            let _ = tx.send(event);
        }
    }

    pub fn set_ble_event_tx(&self, tx: mpsc::Sender<BleEvent>) {
        *self.ble_event_tx.lock().unwrap() = Some(tx);
    }

    pub fn send_ble_event(&self, event: BleEvent) {
        if let Some(tx) = self.ble_event_tx.lock().unwrap().as_ref() {
            let _ = tx.send(event);
        }
    }
}
