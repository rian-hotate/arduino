use std::sync::Arc;
use std::thread::{self, JoinHandle};

use esp_idf_hal::delay::FreeRtos;

use super::Tasks;
use crate::app::ble::BleState;
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
                let mut last_conn = BleState::Idle;

                loop {
                    let conn = tasks.get_ble_conn_state();
                    if conn != last_conn {
                        let led = match conn {
                            BleState::Pairing => tasks,
                            BleState::Connected => LedState::On,
                            BleState::Disconnected => LedState::BlinkingSlow,
                            BleState::Idle => LedState::Off,
                            BleState::Error => LedState::Error,
                        };
                        last_conn = conn;
                    }

                    FreeRtos::delay_ms(50);
                }
            })
            .map_err(|e| Error::new_unexpected(&format!("failed to spawn ui_task: {e}")))?;

        Ok(Self { _handle: h })
    }
}
