mod app;
mod config;
mod common;

use esp_idf_sys::link_patches;

use crate::app::tasks;

fn main() -> anyhow::Result<()> {
    link_patches();

    // タスク起動（ここで全部立ち上げる）
    tasks::start()?;

    // mainは生かしておく（タスク側が動き続ける）
    loop {
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}