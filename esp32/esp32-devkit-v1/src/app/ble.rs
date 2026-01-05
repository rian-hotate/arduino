use esp32_nimble::{
    utilities::mutex::Mutex, uuid128, BLEAdvertisementData, BLEAdvertising, BLEDevice, BLEServer,
    NimbleProperties,
};

// src/app/ble.rs
use crate::common::{Error, Result};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConnState {
    Idle = 0,
    Pairing = 1,
    Connected = 2,
    Disconnected = 3,
    Error = 255,
}

impl From<u8> for ConnState {
    fn from(v: u8) -> Self {
        match v {
            1 => ConnState::Pairing,
            2 => ConnState::Connected,
            3 => ConnState::Disconnected,
            255 => ConnState::Error,
            _ => ConnState::Idle,
        }
    }
}

/// BLE(ペリフェラル)を操作するための薄いラッパ
/// - 実装の中身は今後 (NimBLE / Bluedroid 等) に差し替え可能
pub struct Ble {
    advertising: bool,
    server: Option<&'static mut BLEServer>,
    advertiser: Option<&'static Mutex<BLEAdvertising>>,
}

impl Ble {
    pub fn new() -> Self {
        Self {
            advertising: false,
            advertiser: None,
            server: None,
        }
    }

    /// BLEスタック初期化（1回だけ呼ばれる想定）
    pub fn init(&mut self) -> Result<()> {
        if self.advertiser.is_some() {
            return Ok(());
        }

        let device = BLEDevice::take();
        let server = device.get_server();
        let advertiser = device.get_advertising();

        // ===== GATT Service（最小）=====
        let service = server.create_service(uuid128!("9b574847-f706-436c-bed7-fc01eb0965c1"));
        let chr = service.lock().create_characteristic(
            uuid128!("681285a6-247f-48c6-80ad-68c3dce18585"),
            NimbleProperties::READ,
        );
        chr.lock().set_value(b"hello");

        // ===== Advertise データ =====
        advertiser
            .lock()
            .set_data(
                BLEAdvertisementData::new()
                    .name("esp32-devkit-v1")
                    .add_service_uuid(uuid128!("9b574847-f706-436c-bed7-fc01eb0965c1")),
            )
            .map_err(|e| Error::new_esp(&format!("set adv data failed: {e:?}")))?;

        self.server = Some(server);
        self.advertiser = Some(advertiser);

        Ok(())
    }

    /// ペアリング(アドバタイズ)を開始
    pub fn start_pairing(&mut self) -> Result<()> {
        self.init()?;

        if self.advertising {
            return Ok(());
        }

        if let Some(adv) = &self.advertiser {
            adv.lock()
                .start()
                .map_err(|e| Error::new_esp(&format!("adv start failed: {e:?}")))?;
            self.advertising = true;
        }
        Ok(())
    }

    /// ペアリング(アドバタイズ)停止
    pub fn stop_pairing(&mut self) -> Result<()> {
        if !self.advertising {
            return Ok(());
        }

        if let Some(adv) = &self.advertiser {
            let _ = adv.lock().stop();
        }
        self.advertising = false;
        Ok(())
    }

    /// 接続状態に応じた後処理（任意：必要になったら）
    pub fn on_connected(&mut self) -> Result<()> {
        // TODO: 接続後の処理
        Ok(())
    }

    pub fn on_disconnected(&mut self) -> Result<()> {
        // TODO: 切断後の処理
        Ok(())
    }
}
