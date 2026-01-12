use crate::{app::led::led_event::LedEvent, common::Result};
use std::sync::mpsc;

pub struct LedRunner {}

impl LedRunner {
    fn new(evt_tx: &mpsc::Sender<LedEvent>) -> Self {
        Self {}
    }

    fn start_blinking(&mut self) -> Result<()> {
        // adv.start() など
        Ok(())
    }
}
