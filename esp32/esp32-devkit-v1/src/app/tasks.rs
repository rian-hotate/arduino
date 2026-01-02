use super::led::Led;
use crate::common::Result;
use crate::config::pins::Pins;

pub fn start() -> Result<()> {
    let pins = Pins::take()?;
    let mut led = Led::new(pins.led);

    led.blink(500, 500)
}
