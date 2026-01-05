use crate::common::{Error, Result};
use esp_idf_hal::gpio::{Gpio12, Gpio14, Input, Output, PinDriver, Pull};
use esp_idf_hal::peripherals::Peripherals;

pub struct Pins {
    pub led: PinDriver<'static, Gpio12, Output>,
    pub button: PinDriver<'static, Gpio14, Input>,
}

impl Pins {
    pub fn take() -> Result<Self> {
        let peripherals = Peripherals::take()
            .map_err(|e| Error::new_esp(&format!("failed to take peripherals: {e}")))?;

        let led = PinDriver::output(peripherals.pins.gpio12)
            .map_err(|e| Error::new_esp(&format!("failed to output: {e}")))?;

        let mut button = PinDriver::input(peripherals.pins.gpio14)
            .map_err(|e| Error::new_esp(&format!("failed to init button pin: {e}")))?;
        button
            .set_pull(Pull::Up)
            .map_err(|e| Error::new_esp(&format!("failed to set button pullup: {e}")))?;

        Ok(Self { led, button })
    }
}
