use std::sync::Arc;
use std::thread::{self, JoinHandle};

use esp_idf_hal::delay::FreeRtos;

use crate::app::led::Led;
use crate::app::tasks::Tasks;
use crate::common::{Error, Result};

/// LEDタスク（スレッド）を表す構造体
pub struct LedTask {
    // join したいなら保持。join しないなら _handle でもOK
    _handle: JoinHandle<()>,
}

impl LedTask {
    /// タスク起動（※APIはこのメソッドだけ）
    pub fn start(tasks: Arc<Tasks>, mut led: Led) -> Result<Self> {
        let handle = thread::Builder::new()
            .name("led_task".to_string())
            .spawn(move || {
                // ループ: Tasks の状態を読んで LED の挙動を変える
                // エラーになったら落とさず Error状態に寄せるなど
                loop {
                    let state = tasks.get_led_state();
                    if let Err(_e) = led.tick(state) {
                        // フェイルセーフ
                        let _ = led.set_state(crate::app::led::LedState::Error);
                        FreeRtos::delay_ms(50);
                    }
                    // tick内で delay するならここは軽くてもOK
                    FreeRtos::delay_ms(1);
                }
            })
            .map_err(|e| Error::new_unexpected(&format!("failed to spawn led_task: {e}")))?;

        Ok(Self { _handle: handle })
    }

    /// 必要なら停止APIも後から追加できる（Atomicのstop flag等）
    pub fn is_running(&self) -> bool {
        // Rust標準のJoinHandleには「生存チェック」がないので、
        // ここは stop flag 実装後に意味を持たせるのが現実的
        true
    }
}
