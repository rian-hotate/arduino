use std::sync::mpsc;

use esp32_nimble::BLEDevice;

use crate::app::ble::ble_event::BleEvent;
use crate::common::{Error, Result};

struct BleRunner {
    // ここに server / adv / service などを持つ
    // server: BLEServer,
    // adv: esp32_nimble::BLEAdvertising, or Mutex<...> etc
}

impl BleRunner {
    fn new(evt_tx: &mpsc::Sender<BleEvent>) -> Self {
        BLEDevice::init();

        // ここで server/service/characteristic/callback をセット
        // 接続/切断イベントが来たら evt_tx.send(BleEvent::Connected) など

        Self {}
    }

    fn start_advertise(&mut self) -> Result<()> {
        // adv.start() など
        Ok(())
    }

    fn stop_advertise(&mut self) -> Result<()> {
        // adv.stop() など
        Ok(())
    }

    fn disconnect_all(&mut self) -> Result<()> {
        Ok(())
    }
}
