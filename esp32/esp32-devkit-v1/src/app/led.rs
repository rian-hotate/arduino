use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::{Gpio13, Output, PinDriver};

use crate::common::{Error, Result};

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum LedState {
    BlinkingFast = 0,
    BlinkingSlow = 1,
    On = 2,
    Off = 3,
    Error = 4,
}
impl From<u8> for LedState {
    fn from(v: u8) -> Self {
        match v {
            0 => LedState::BlinkingFast,
            1 => LedState::BlinkingSlow,
            2 => LedState::On,
            3 => LedState::Off,
            4 => LedState::Error,
            _ => LedState::Off,
        }
    }
}

pub struct Led {
    pin: PinDriver<'static, Gpio13, Output>,
    state: LedState,
    phase_on: bool, // 点滅のON/OFF位相
}

impl Led {
    pub fn new(pin: PinDriver<'static, Gpio13, Output>) -> Self {
        Self {
            pin,
            state: LedState::Off,
            phase_on: false,
        }
    }

    pub fn set_state(&mut self, state: LedState) -> Result<()> {
        self.state = state;
        Ok(())
    }

    pub fn tick(&mut self, state: LedState) -> Result<()> {
        // 外から渡された state を優先して採用
        self.state = state;

        match self.state {
            LedState::On => {
                self.on()?;
                FreeRtos::delay_ms(10);
                Ok(())
            }
            LedState::Off => {
                self.off()?;
                FreeRtos::delay_ms(10);
                Ok(())
            }
            LedState::BlinkingFast => self.blink_step(200),
            LedState::BlinkingSlow => self.blink_step(1000),
            LedState::Error => self.blink_step(80),
        }
    }

    fn blink_step(&mut self, interval_ms: u32) -> Result<()> {
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

    fn on(&mut self) -> Result<()> {
        self.pin
            .set_high()
            .map_err(|e| Error::new_invalid_state(&format!("failed to set LED HIGH: {e}")))?;
        Ok(())
    }

    fn off(&mut self) -> Result<()> {
        self.pin
            .set_low()
            .map_err(|e| Error::new_invalid_state(&format!("failed to set LED LOW: {e}")))?;
        Ok(())
    }
}
