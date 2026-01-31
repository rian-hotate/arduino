use esp_idf_hal::delay::FreeRtos;

use crate::{
    app::{
        ble::{ble_command::BleCommand, ble_event::BleEvent, ble_handle::BleHandle, Ble},
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
    #[allow(dead_code)]
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
                let event_tasks = tasks.clone();
                ble.set_event_sink(Arc::new(move |event| {
                    event_tasks.send_ble_event(event);
                }));
                let mut pairing_deadline: Option<Instant> = None;

                loop {
                    // コマンド処理
                    while let Ok(cmd) = rx.try_recv() {
                        match cmd {
                            BleCommand::StartAdvertise { timeout_ms } => {
                                match ble.start_pairing() {
                                    Ok(()) => {
                                        tasks.send_ble_event(BleEvent::AdvertisingStarted);
                                        pairing_deadline = Some(
                                            Instant::now()
                                                + Duration::from_millis(timeout_ms as u64),
                                        );
                                    }
                                    Err(e) => {
                                        tasks.send_ble_event(BleEvent::Error);
                                        log::error!("failed to start pairing: {e}");
                                    }
                                }
                            }
                            BleCommand::StopAdvertise => {
                                let _ = ble.stop_pairing();
                                tasks.send_ble_event(BleEvent::AdvertisingStopped);
                                pairing_deadline = None;
                            }
                            BleCommand::Shutdown => {
                                let _ = ble.stop_pairing();
                                let _ = ble.on_disconnected();
                                return;
                            }
                        }
                    }

                    // タイムアウト処理
                    if let Some(deadline) = pairing_deadline {
                        if Instant::now() >= deadline {
                            let _ = ble.stop_pairing();
                            tasks.send_ble_event(BleEvent::AdvertisingStopped);
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
