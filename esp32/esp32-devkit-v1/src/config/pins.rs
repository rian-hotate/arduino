use crate::common::{Error, Result};
use esp_idf_hal::gpio::{Gpio33, Output, PinDriver};
use esp_idf_hal::peripherals::Peripherals;

pub struct Pins {
    pub led: PinDriver<'static, Gpio33, Output>,
}

impl Pins {
    pub fn take() -> Result<Self> {
        let peripherals = Peripherals::take()
            .map_err(|e| Error::new_esp(&format!("failed to take peripherals: {e}")))?;

        let led = PinDriver::output(peripherals.pins.gpio33)
            .map_err(|e| Error::new_esp(&format!("failed to output: {e}")))?;

        Ok(Self { led })
    }
}
