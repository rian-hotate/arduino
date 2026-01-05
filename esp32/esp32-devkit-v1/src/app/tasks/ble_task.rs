// src/tasks/ble_task.rs
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use esp_idf_hal::delay::FreeRtos;

use super::Tasks;
use crate::app::ble::{Ble, ConnState};
use crate::app::led::LedState;
use crate::common::{Error, Result};

pub struct BleTask {
    _handle: JoinHandle<()>,
}

impl BleTask {
    pub fn start(tasks: Arc<Tasks>) -> Result<Self> {
        let h = thread::Builder::new()
            .name("ble_task".into())
            .stack_size(8192)
            .spawn(move || {
                let mut ble = Ble::new();
                let mut pairing_started_at: Option<Instant> = None;

                // 初期化（失敗したらログ出したいけど、今は握りつぶし例）
                if let Err(_e) = ble.init() {
                    // 失敗したら状態をErrorに落とす等も可
                    // tasks.set_conn_state(ConnState::Idle);
                }

                let mut prev = tasks.get_ble_conn_state();

                loop {
                    let cur = tasks.get_ble_conn_state();

                    if cur != prev {
                        // 状態遷移に応じた処理
                        match cur {
                            ConnState::Pairing => {
                                // ペアリング開始
                                let _ = ble.start_pairing();
                                pairing_started_at = Some(Instant::now());
                                tasks.set_led_state(LedState::BlinkingFast);
                            }
                            ConnState::Connected => {
                                let _ = ble.on_connected();
                                pairing_started_at = None;
                                tasks.set_led_state(LedState::On);
                            }
                            ConnState::Disconnected => {
                                let _ = ble.on_disconnected();
                                pairing_started_at = None;
                                tasks.set_led_state(LedState::Off);
                            }
                            ConnState::Idle => {
                                let _ = ble.stop_pairing();
                                pairing_started_at = None;
                                tasks.set_led_state(LedState::Off);
                            }
                            ConnState::Error => {
                                // エラー状態の処理
                                let _ = ble.stop_pairing();
                                pairing_started_at = None;
                                tasks.set_led_state(LedState::Error);
                            }
                        }

                        prev = cur;
                    }

                    if let (ConnState::Pairing, Some(start)) = (cur, pairing_started_at) {
                        if start.elapsed() >= Duration::from_secs(60) {
                            log::info!("Pairing timeout (60s), stop advertising");
                            let _ = ble.stop_pairing();
                            pairing_started_at = None;
                            tasks.set_ble_conn_state(ConnState::Idle);
                        }
                    }

                    FreeRtos::delay_ms(50);
                }
            })
            .map_err(|e| Error::new_unexpected(&format!("failed to spawn ble_task: {e}")))?;

        Ok(Self { _handle: h })
    }
}
