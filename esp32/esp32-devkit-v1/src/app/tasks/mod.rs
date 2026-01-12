pub mod ble_task;
pub mod button_task;
pub mod ui_task;

use core::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;

use crate::app::{
    ble::BleState,
    button::Button,
    led::{led_handle::LedHandle, led_task::LedTask, Led},
    tasks::{ble_task::BleTask, ui_task::UiTask},
};
use crate::common::Result;
use crate::config::pins::Pins;

/// タスク間共有の状態を持つ構造体
pub struct Tasks {
    conn_state: AtomicU8,
}

impl Tasks {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            conn_state: AtomicU8::new(BleState::Idle as u8),
        })
    }
}

/// タスク起動の入口
pub struct TaskManager {
    pub tasks: Arc<Tasks>,

    led_task: Option<LedTask>,
    pub led_handler: Option<LedHandle>,
    button_task: Option<button_task::ButtonTask>,
    ble_task: Option<ble_task::BleTask>,
    ui_task: Option<ui_task::UiTask>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: Tasks::new(),
            led_handler: None,
            led_task: None,
            button_task: None,
            ble_task: None,
            ui_task: None,
        }
    }

    pub fn start(&mut self) -> Result<()> {
        let pins = Pins::take()?;
        let led = Led::new(pins.led);
        let button = Button::new(pins.button)?;

        self.start_led_task(led)?;
        self.start_button_task(button)?;
        self.start_ble_task()?;
        self.start_ui_task()?;

        Ok(())
    }

    fn start_led_task(&mut self, led: crate::app::led::Led) -> Result<()> {
        let (led_task, led_handle) = LedTask::start(led)?;
        self.led_task = Some(led_task);
        self.led_handler = Some(led_handle.clone());

        Ok(())
    }

    fn start_button_task(&mut self, button: crate::app::button::Button) -> Result<()> {
        let t = button_task::ButtonTask::start(self.tasks.clone(), button)?;
        self.button_task = Some(t);
        Ok(())
    }

    fn start_ble_task(&mut self) -> Result<()> {
        let t = BleTask::start(self.tasks.clone())?;
        self.ble_task = Some(t);
        Ok(())
    }

    fn start_ui_task(&mut self) -> Result<()> {
        let t = UiTask::start(self.tasks.clone())?;
        self.ui_task = Some(t);
        Ok(())
    }
}
