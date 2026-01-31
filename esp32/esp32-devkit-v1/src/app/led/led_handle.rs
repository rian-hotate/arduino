use std::sync::mpsc::Sender;

use crate::app::led::led_command::LedCommand;

/// 外部公開：LED状態を送るためのハンドル（Queue送信のみ）
#[derive(Clone)]
pub struct LedHandle {
    #[allow(dead_code)]
    pub tx: Sender<LedCommand>,
}

impl LedHandle {
    #[allow(dead_code)]
    fn set(&self, state: LedCommand) {
        let _ = self.tx.send(state);
    }

    #[allow(dead_code)]
    pub fn queue_blinking(&self, interval_ms: u32) {
        self.set(LedCommand::Blink { interval_ms });
    }

    #[allow(dead_code)]
    pub fn queue_on(&self) {
        self.set(LedCommand::On);
    }

    #[allow(dead_code)]
    pub fn queue_off(&self) {
        self.set(LedCommand::Off);
    }
}
