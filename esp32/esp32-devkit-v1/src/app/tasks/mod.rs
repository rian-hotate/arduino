pub mod button_task;
pub mod led_task;

use core::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;

use crate::app::{
    button::Button,
    led::{Led, LedState},
    tasks::led_task::LedTask,
};
use crate::common::Result;
use crate::config::pins::Pins;

/// タスク間共有の状態を持つ構造体
pub struct Tasks {
    led_state: AtomicU8,
}

impl Tasks {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            led_state: AtomicU8::new(LedState::Off as u8),
        })
    }

    pub fn set_led_state(&self, state: LedState) {
        self.led_state.store(state as u8, Ordering::Relaxed);
    }

    pub fn get_led_state(&self) -> LedState {
        LedState::from(self.led_state.load(Ordering::Relaxed))
    }
}

/// タスク起動の入口
pub struct TaskManager {
    pub tasks: Arc<Tasks>,
    led_task: Option<led_task::LedTask>,
    button_task: Option<button_task::ButtonTask>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: Tasks::new(),
            led_task: None,
            button_task: None,
        }
    }

    pub fn start(&mut self) -> Result<()> {
        // pins.rsで作った PinDriver を使って Led を作る
        let pins = Pins::take()?; // あなたの実装
        let led = Led::new(pins.led);
        let button = Button::new(pins.button)?;

        self.start_led_task(led)?;
        self.start_button_task(button)
    }

    /// LEDタスクを起動して保持する
    fn start_led_task(&mut self, led: crate::app::led::Led) -> Result<()> {
        let t = LedTask::start(self.tasks.clone(), led)?;
        self.led_task = Some(t);
        Ok(())
    }

    pub fn start_button_task(&mut self, button: crate::app::button::Button) -> Result<()> {
        let t = button_task::ButtonTask::start(self.tasks.clone(), button)?;
        self.button_task = Some(t);
        Ok(())
    }
}
