pub mod led_command;
mod led_event;
pub mod led_handle;
mod led_runner;
pub mod led_task;

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::{Gpio12, Output, PinDriver};

use crate::app::led::led_command::LedCommand;
use crate::common::{Error, Result};

pub struct Led {
    #[allow(dead_code)]
    pin: PinDriver<'static, Gpio12, Output>,
    #[allow(dead_code)]
    phase_on: bool, // 点滅のON/OFF位相
}

impl Led {
    pub fn new(pin: PinDriver<'static, Gpio12, Output>) -> Self {
        Self {
            pin,
            phase_on: false,
        }
    }

    #[allow(dead_code)]
    fn tick(&mut self, interval_ms: u32) -> Result<()> {
        // 位相を反転して1ステップだけ実行

        self.phase_on = !self.phase_on;
        if self.phase_on {
            self.on()?;
        } else {
            self.off()?;
        }
        FreeRtos::delay_ms(interval_ms);
        Ok(())
    }

    #[allow(dead_code)]
    pub fn on(&mut self) -> Result<()> {
        self.pin
            .set_high()
            .map_err(|e| Error::new_invalid_state(&format!("failed to set LED HIGH: {e}")))?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn off(&mut self) -> Result<()> {
        self.pin
            .set_low()
            .map_err(|e| Error::new_invalid_state(&format!("failed to set LED LOW: {e}")))?;
        Ok(())
    }
}
