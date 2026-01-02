use std::sync::Arc;
use std::thread::{self, JoinHandle};

use esp_idf_hal::delay::FreeRtos;

use crate::app::button::Button;
use crate::app::led::LedState;
use crate::app::tasks::Tasks;
use crate::common::{Error, Result};

pub struct ButtonTask {
    _handle: JoinHandle<()>,
}

impl ButtonTask {
    pub fn start(tasks: Arc<Tasks>, mut button: Button) -> Result<Self> {
        let handle = thread::Builder::new()
            .name("button_task".to_string())
            .spawn(move || loop {
                if button.poll_pressed_edge() {
                    let next = match tasks.get_led_state() {
                        LedState::Off => LedState::BlinkingSlow,
                        LedState::BlinkingSlow => LedState::BlinkingFast,
                        LedState::BlinkingFast => LedState::On,
                        LedState::On => LedState::Off,
                        LedState::Error => LedState::Off,
                    };
                    tasks.set_led_state(next);
                }
                FreeRtos::delay_ms(10);
            })
            .map_err(|e| Error::new_unexpected(&format!("failed to spawn button_task: {e}")))?;

        Ok(Self { _handle: handle })
    }

    /// 必要なら停止APIも後から追加できる（Atomicのstop flag等）
    pub fn is_running(&self) -> bool {
        // Rust標準のJoinHandleには「生存チェック」がないので、
        // ここは stop flag 実装後に意味を持たせるのが現実的
        true
    }
}
