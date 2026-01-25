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
        *self.led_handle.lock().expect("led_handle mutex poisoned") = Some(handle);
    }

    pub fn set_ble_handle(&self, handle: BleHandle) {
        *self.ble_handle.lock().expect("ble_handle mutex poisoned") = Some(handle);
    }

    pub fn send_led_command(&self, cmd: crate::app::led::led_command::LedCommand) {
        match self
            .led_handle
            .lock()
            .expect("led_handle mutex poisoned")
            .as_ref()
        {
            Some(handle) => {
                if let Err(e) = handle.tx.send(cmd) {
                    eprintln!("failed to send led command: {e}");
                }
            }
            None => eprintln!("led handle not set; dropping led command"),
        }
    }

    #[allow(dead_code)]
    pub fn send_ble_command(&self, cmd: crate::app::ble::ble_command::BleCommand) {
        match self
            .ble_handle
            .lock()
            .expect("ble_handle mutex poisoned")
            .as_ref()
        {
            Some(handle) => {
                if let Err(e) = handle.tx.send(cmd) {
                    eprintln!("failed to send ble command: {e}");
                }
            }
            None => eprintln!("ble handle not set; dropping ble command"),
        }
    }

    pub fn set_button_event_tx(&self, tx: mpsc::Sender<ButtonEvent>) {
        *self
            .button_event_tx
            .lock()
            .expect("button_event_tx mutex poisoned") = Some(tx);
    }

    pub fn send_button_event(&self, event: ButtonEvent) {
        match self
            .button_event_tx
            .lock()
            .expect("button_event_tx mutex poisoned")
            .as_ref()
        {
            Some(tx) => {
                if let Err(e) = tx.send(event) {
                    eprintln!("failed to send button event: {e}");
                }
            }
            None => eprintln!("button event channel not set; dropping event"),
        }
    }

    pub fn set_ble_event_tx(&self, tx: mpsc::Sender<BleEvent>) {
        *self
            .ble_event_tx
            .lock()
            .expect("ble_event_tx mutex poisoned") = Some(tx);
    }

    pub fn send_ble_event(&self, event: BleEvent) {
        match self
            .ble_event_tx
            .lock()
            .expect("ble_event_tx mutex poisoned")
            .as_ref()
        {
            Some(tx) => {
                if let Err(e) = tx.send(event) {
                    eprintln!("failed to send ble event: {e}");
                }
            }
            None => eprintln!("ble event channel not set; dropping event"),
        }
    }
}
