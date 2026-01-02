use crate::common::{Error, Result};

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::{Gpio33, Output, PinDriver};

/// LED 制御用構造体
pub struct Led {
    pin: PinDriver<'static, Gpio33, Output>,
}

impl Led {
    /// 新しい LED インスタンスを作成
    pub fn new(pin: PinDriver<'static, Gpio33, Output>) -> Self {
        Self { pin }
    }

    /// LED ON
    pub fn on(&mut self) -> Result<()> {
        self.pin
            .set_high()
            .map_err(|e| Error::new_invalid_state(&format!("failed to set LED HIGH: {e}")))
    }

    /// LED OFF
    pub fn off(&mut self) -> Result<()> {
        self.pin
            .set_low()
            .map_err(|e| Error::new_invalid_state(&format!("failed to set LED LOW: {e}")))
    }

    /// 点滅（ブロッキング）
    pub fn blink(&mut self, on_ms: u32, off_ms: u32) -> Result<()> {
        loop {
            self.on()?;
            FreeRtos::delay_ms(on_ms);
            self.off()?;
            FreeRtos::delay_ms(off_ms);
        }
    }
}
