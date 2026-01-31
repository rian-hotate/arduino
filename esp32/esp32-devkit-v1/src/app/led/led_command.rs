#[derive(PartialEq, Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum LedCommand {
    Blink { interval_ms: u32 },
    On,
    Off,
    GetStatus,
    Shutdown,
}
