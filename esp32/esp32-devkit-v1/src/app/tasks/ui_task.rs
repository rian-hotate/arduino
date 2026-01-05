use std::sync::Arc;
use std::thread::{self, JoinHandle};

use esp_idf_hal::delay::FreeRtos;

use super::Tasks;
use crate::app::ble::ConnState;
use crate::app::led::LedState;
use crate::common::{Error, Result};

pub struct UiTask {
    _handle: JoinHandle<()>,
}

impl UiTask {
    pub fn start(tasks: Arc<Tasks>) -> Result<Self> {
        let h = thread::Builder::new()
            .name("ui_task".into())
            .stack_size(4096)
            .spawn(move || {
                // 前回値を持って、変化があった時だけLED状態を書き換える
                let mut last_conn = ConnState::Idle;

                loop {
                    let conn = tasks.get_ble_conn_state();
                    if conn != last_conn {
                        let led = match conn {
                            ConnState::Pairing => LedState::BlinkingFast,
                            ConnState::Connected => LedState::On,
                            ConnState::Disconnected => LedState::BlinkingSlow,
                            ConnState::Idle => LedState::Off,
                            ConnState::Error => LedState::Error,
                        };
                        tasks.set_led_state(led);
                        last_conn = conn;
                    }

                    FreeRtos::delay_ms(50);
                }
            })
            .map_err(|e| Error::new_unexpected(&format!("failed to spawn ui_task: {e}")))?;

        Ok(Self { _handle: h })
    }
}
