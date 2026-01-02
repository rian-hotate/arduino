mod app;
mod common;
mod config;

use esp_idf_sys::link_patches;

use crate::{app::tasks::TaskManager, common::Result};

fn main() -> Result<()> {
    link_patches();

    let mut manager = TaskManager::new();
    manager.start()?;

    // mainは生かしておく（タスク側が動き続ける）
    loop {
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}
