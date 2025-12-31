use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;

fn main() -> anyhow::Result<()> {
    // ESP-IDF 初期化
    esp_idf_sys::link_patches();

    // 周辺機器取得
    let peripherals = Peripherals::take().unwrap();

    // GPIO33 を出力に設定
    let mut led = PinDriver::output(peripherals.pins.gpio33)?;

    loop {
        // LED ON
        led.set_high()?;
        FreeRtos::delay_ms(5000);

        // LED OFF
        led.set_low()?;
        FreeRtos::delay_ms(5000);
    }
}
