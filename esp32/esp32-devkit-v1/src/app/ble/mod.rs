pub mod ble_command;
pub mod ble_event;
pub mod ble_handle;
pub mod ble_task;

use esp32_nimble::{
    utilities::mutex::Mutex, uuid128, BLEAdvertisementData, BLEAdvertising, BLEDevice, BLEServer,
    NimbleProperties,
};
use std::sync::Arc;

use crate::app::ble::ble_event::BleEvent;
use crate::common::{Error, Result};
use crate::config::ble::BleConfig;

pub struct Ble {
    server: Option<&'static mut BLEServer>,
    advertiser: Option<&'static Mutex<BLEAdvertising>>,
    event_sink: Option<Arc<dyn Fn(BleEvent) + Send + Sync>>,
}

impl Ble {
    pub fn new() -> Self {
        Self {
            advertiser: None,
            server: None,
            event_sink: None,
        }
    }

    pub fn set_event_sink(&mut self, sink: Arc<dyn Fn(BleEvent) + Send + Sync>) {
        self.event_sink = Some(sink);
    }

    /// BLEスタック初期化（1回だけ呼ばれる想定）
    pub fn init(&mut self) -> Result<()> {
        if self.advertiser.is_some() {
            log::debug!("BLE already initialized");
            return Ok(());
        }

        log::info!("BLE initializing...");
        let device = BLEDevice::take();
        let server = device.get_server();
        let advertiser = device.get_advertising();

        // 切断時の自動アドバタイズ再開を無効化
        server.advertise_on_disconnect(false);

        // ===== GATT Service（最小）=====
        let service = server.create_service(uuid128!(BleConfig::SERVICE_UUID));
        let chr = service.lock().create_characteristic(
            uuid128!(BleConfig::CHARACTERISTIC_UUID),
            NimbleProperties::READ,
        );

        // キャラクタリスティックに値を設定
        chr.lock().set_value(b"hello");

        log::info!(
            "GATT service created: {}, characteristic: {}",
            BleConfig::SERVICE_UUID,
            BleConfig::CHARACTERISTIC_UUID
        );
        log::debug!("GATT service and characteristic created");

        if let Some(sink) = self.event_sink.as_ref().cloned() {
            let connect_sink = sink.clone();
            let disconnect_sink = sink.clone();
            let advertiser_on_connect = advertiser;

            server.on_connect(move |_, _| {
                log::info!("BLE device connected - auto-stopping advertising");
                let _ = advertiser_on_connect.lock().stop();
                (connect_sink)(BleEvent::Connected);
            });

            server.on_disconnect(move |_, _| {
                log::info!("BLE device disconnected");
                (disconnect_sink)(BleEvent::Disconnected);
            });
            log::debug!("Connection callbacks registered");
        } else {
            log::warn!("Event sink not set, callbacks not registered");
        }

        // ===== Advertise データ =====
        advertiser
            .lock()
            .set_data(
                BLEAdvertisementData::new()
                    .name(BleConfig::DEVICE_NAME)
                    .add_service_uuid(uuid128!(BleConfig::SERVICE_UUID)),
            )
            .map_err(|e| Error::new_esp(&format!("set adv data failed: {e:?}")))?;
        log::debug!("Advertisement data configured");

        self.server = Some(server);
        self.advertiser = Some(advertiser);
        log::info!("BLE initialization completed");

        Ok(())
    }

    /// ペアリング(アドバタイズ)を開始
    pub fn start_pairing(&mut self) -> Result<()> {
        log::debug!("start_pairing called");
        self.init()?;

        if let Some(adv) = &self.advertiser {
            adv.lock()
                .start()
                .map_err(|e| Error::new_esp(&format!("adv start failed: {e:?}")))?;
            log::info!("Advertising started");
        }
        Ok(())
    }

    /// ペアリング(アドバタイズ)停止
    pub fn stop_pairing(&mut self) -> Result<()> {
        log::debug!("stop_pairing called");
        if let Some(adv) = &self.advertiser {
            let _ = adv.lock().stop();
        }
        log::info!("Advertising stopped");
        Ok(())
    }

    pub fn on_disconnected(&mut self) -> Result<()> {
        // TODO: 切断後の処理
        Ok(())
    }

    /// 現在のBLE接続状態を取得
    pub fn is_connected(&self) -> bool {
        if let Some(server) = &self.server {
            server.connected_count() > 0
        } else {
            false
        }
    }
}
