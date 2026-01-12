use esp_idf_hal::delay::FreeRtos;

use crate::{
    app::{
        ble::{ble_command::BleCommand, ble_handle::BleHandle, ble_state::BleState, Ble},
        tasks::Tasks,
    },
    common::{Error, Result},
};
use std::{
    sync::{mpsc, Arc},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

pub struct BleTask {
    handle: JoinHandle<()>,
}

impl BleTask {
    pub fn start(tasks: Arc<Tasks>) -> Result<(Self, BleHandle)> {
        let (tx, rx) = mpsc::channel::<BleCommand>();

        let handle = thread::Builder::new()
            .name("ble_task".into())
            .stack_size(8192)
            .spawn(move || {
                let mut ble = Ble::new();
                let mut pairing_deadline: Option<Instant> = None;

                loop {
                    // コマンド処理
                    while let Ok(cmd) = rx.try_recv() {
                        match cmd {
                            BleCommand::StartAdvertise { timeout_ms } => {
                                if ble.start_pairing().is_ok() {
                                    tasks.set_ble_conn_state(BleState::Advertising);
                                    pairing_deadline = Some(
                                        Instant::now() + Duration::from_millis(timeout_ms as u64),
                                    );
                                }
                            }
                            BleCommand::StopAdvertise => {
                                let _ = ble.stop_pairing();
                                tasks.set_ble_conn_state(BleState::Idle);
                                pairing_deadline = None;
                            }
                            BleCommand::Shutdown => {
                                let _ = ble.on_disconnected();
                                tasks.set_ble_conn_state(BleState::Idle);
                                return;
                            }
                            _ => { /* 他コマンドは未実装 */ }
                        }
                    }

                    // 60秒タイムアウト
                    if let Some(deadline) = pairing_deadline {
                        if Instant::now() >= deadline {
                            let _ = ble.stop_pairing();
                            tasks.set_ble_conn_state(BleState::Idle);
                            pairing_deadline = None;
                        }
                    }

                    FreeRtos::delay_ms(20);
                }
            })
            .map_err(|e| Error::new_unexpected(&format!("failed to spawn ble_task: {e}")))?;

        Ok((Self { handle }, BleHandle { tx }))
    }
}
