use crate::common::{Error, Result};

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::{Gpio14, Input, PinDriver, Pull};

/// ボタン（Gpio14 / Active-Low：押すとLOW）
pub struct Button {
    pin: PinDriver<'static, Gpio14, Input>,
    last_low: bool,
}

impl Button {
    pub fn new(mut pin: PinDriver<'static, Gpio14, Input>) -> Result<Self> {
        pin.set_pull(Pull::Up)
            .map_err(|e| Error::new_invalid_state(&format!("failed to set pull-up: {e}")))?;

        Ok(Self {
            pin,
            last_low: false,
        })
    }

    pub fn is_pressed(&self) -> bool {
        self.pin.is_low()
    }

    /// 押された瞬間だけ true（簡易デバウンス）
    pub fn poll_pressed_edge(&mut self) -> bool {
        let now_low = self.is_pressed();
        let edge = now_low && !self.last_low;

        if edge {
            FreeRtos::delay_ms(30);
            if self.is_pressed() {
                self.last_low = true;
                return true;
            }
        }

        self.last_low = now_low;
        false
    }
}
