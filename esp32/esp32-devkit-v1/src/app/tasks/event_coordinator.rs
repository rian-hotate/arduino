use std::sync::{mpsc, Arc};
use std::thread::{self, JoinHandle};

use esp_idf_hal::delay::FreeRtos;

use super::Tasks;
use crate::app::ble::{ble_command::BleCommand, ble_event::BleEvent};
use crate::app::button::event::ButtonEvent;
use crate::app::led::led_command::LedCommand;
use crate::common::{Error, Result};

/// イベント集約・制御タスク
/// 各タスクからのイベントを受信し、システム全体を調整する
pub struct EventCoordinator {
    _handle: JoinHandle<()>,
}

impl EventCoordinator {
    pub fn start(
        tasks: Arc<Tasks>,
    ) -> Result<(Self, mpsc::Sender<ButtonEvent>, mpsc::Sender<BleEvent>)> {
        let (button_tx, button_rx) = mpsc::channel::<ButtonEvent>();
        let (ble_tx, ble_rx) = mpsc::channel::<BleEvent>();

        let h = thread::Builder::new()
            .name("event_coordinator".into())
            .stack_size(4096)
            .spawn(move || loop {
                // ボタンイベント処理
                while let Ok(event) = button_rx.try_recv() {
                    match event {
                        ButtonEvent::LongPress => {
                            // 長押し：BLEアドバタイズ開始
                            tasks
                                .send_ble_command(BleCommand::StartAdvertise { timeout_ms: 60000 });
                        }
                        ButtonEvent::ShortPress => {
                            // 短押し：将来の拡張用
                        }
                    }
                }

                // BLEイベント処理
                while let Ok(event) = ble_rx.try_recv() {
                    let led_cmd = match event {
                        BleEvent::AdvertisingStarted => LedCommand::Blink { interval_ms: 500 },
                        BleEvent::AdvertisingStopped => LedCommand::Off,
                        BleEvent::Connected => LedCommand::On,
                        BleEvent::Disconnected => LedCommand::Blink { interval_ms: 1000 },
                        BleEvent::Error => LedCommand::Blink { interval_ms: 100 },
                    };
                    // LED タスクのキューにコマンドを送信
                    tasks.send_led_command(led_cmd);
                }

                FreeRtos::delay_ms(20);
            })
            .map_err(|e| {
                Error::new_unexpected(&format!("failed to spawn event_coordinator: {e}"))
            })?;

        Ok((Self { _handle: h }, button_tx, ble_tx))
    }
}
