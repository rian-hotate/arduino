use esp_idf_hal::delay::FreeRtos;

use crate::app::led::led_handle::LedHandle;
use crate::app::led::led_runner::LedRunner;
use crate::app::led::{Led, LedCommand};
use crate::app::tasks::Tasks;
use crate::common::{Error, Result};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};

/// LEDタスク本体（スレッド寿命を保持）
pub struct LedTask {
    handle: JoinHandle<()>,
}

impl LedTask {
    pub fn start(tasks: Arc<Tasks>) -> (Self, LedHandle) {
        let (tx, rx) = mpsc::channel::<LedCommand>();

        let handle = thread::Builder::new()
            .name("led_task".into())
            .stack_size(4096)
            .spawn(move || {
                loop {
                    let mut led = Led::new();
                    // コマンド処理
                    while let Ok(cmd) = rx.try_recv() {
                        match cmd {
                            _ => { /* 他コマンドは未実装 */ }
                        }
                    }

                    FreeRtos::delay_ms(20);
                }
            })
            .unwrap();

        (Self { handle }, LedHandle { mailbox })
    }
}
