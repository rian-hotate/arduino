use crate::{app::led::led_event::LedEvent, common::Result};
use std::sync::mpsc;

#[allow(dead_code)]
pub struct LedRunner {}

#[allow(dead_code)]
impl LedRunner {
    fn new(_evt_tx: &mpsc::Sender<LedEvent>) -> Self {
        Self {}
    }

    fn start_blinking(&mut self) -> Result<()> {
        // adv.start() など
        Ok(())
    }
}
