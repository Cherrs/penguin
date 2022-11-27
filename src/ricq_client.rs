use crate::{AccountInfo, FriendList, LoginState, MessageClientError, QrcodeConfirmed};
use bytes::{BufMut, Bytes, BytesMut};
use rand::Rng;
use ricq::{
    client::{Connector, DefaultConnector},
    ext::common::after_login,
    handler::DefaultHandler,
    Client, Device, LoginResponse, Protocol, QRCodeConfirmed, QRCodeState,
};
use std::{
    path::Path,
    sync::{Arc, Mutex},
};
use tracing::{error, trace};

pub struct RicqClient {
    client: Arc<Client>,
    sig: Mutex<BytesMut>,
}

impl RicqClient {
    /// 初始化Client
    pub async fn init() -> Self {
        let device = match Path::new("device.json").exists() {
            true => serde_json::from_str(
                &tokio::fs::read_to_string("device.json")
                    .await
                    .expect("failed to read device.json"),
            )
            .expect("failed to parse device info"),
            false => {
                let d = build_delives(Device::random());
                tokio::fs::write("device.json", serde_json::to_string(&d).unwrap())
                    .await
                    .expect("failed to write device info to file");
                d
            }
        };
        let client = Arc::new(Client::new(device, Protocol::MacOS.into(), DefaultHandler));
        tokio::spawn({
            let client = client.clone();
            let stream = DefaultConnector.connect(&client).await.unwrap();
            async move { client.start(stream).await }
        });
        tokio::task::yield_now().await;
        trace!("初始化ricq完成");
        RicqClient {
            client,
            sig: Mutex::new(BytesMut::with_capacity(24)),
        }
    }
    /// 获取登录二维码
    pub async fn get_login_qrcode(&self) -> Result<Bytes, MessageClientError> {
        let qrcode_result = self.client.fetch_qrcode().await;
        match qrcode_result {
            Ok(state) => match state {
                QRCodeState::ImageFetch(fetch) => {
                    self.sig.lock().unwrap().put(fetch.sig);
                    Ok(fetch.image_data)
                }
                _ => {
                    error!("没有处理的获取二维码enum");
                    Err(MessageClientError::FetchQrcodeFail)
                }
            },
            Err(e) => {
                error!("获取二维码失败:{}", e);
                Err(MessageClientError::FetchQrcodeFail)
            }
        }
    }
    /// 获取登录状态
    pub async fn get_qrcode_state(&self) -> Result<LoginState, MessageClientError> {
        trace!("获取登录状态");
        let sig = self.sig.lock().unwrap().clone();
        let qrcode_state = self.client.query_qrcode_result(&sig).await;
        trace!("当前登陆状态：{:?}", qrcode_state);
        match qrcode_state {
            Ok(state) => {
                let result = match state {
                    QRCodeState::Canceled => LoginState::Canceled,
                    QRCodeState::Confirmed(t) => {
                        if let LoginResponse::Success(_) = self.login(&t).await? {
                            after_login(&self.client).await;
                            LoginState::Confirmed(QrcodeConfirmed(t.uin))
                        } else {
                            return Err(MessageClientError::LoginFail);
                        }
                    }
                    QRCodeState::ImageFetch(_) => LoginState::Canceled,
                    QRCodeState::Timeout => LoginState::Timeout,
                    QRCodeState::WaitingForConfirm => LoginState::WaitingForConfirm,
                    QRCodeState::WaitingForScan => LoginState::WaitingForScan,
                };
                Ok(result)
            }
            Err(e) => {
                error!("获取扫码状态失败:{}", e);
                Err(MessageClientError::GetQrcodeStateFail)
            }
        }
    }

    /// 获取登录二维码的Base64图片
    pub async fn get_login_qrcode_base64(&self) -> Result<String, MessageClientError> {
        let qrcode = self.get_login_qrcode().await?;
        let base64str = format!("data:image/png;base64,{}", base64::encode(qrcode));
        Ok(base64str)
    }
    /// 获取好友列表
    pub async fn get_friend_list(&self) -> Result<FriendList, MessageClientError> {
        trace!("获取好友列表");
        match self.client.get_friend_list().await {
            Ok(list) => Ok(list.into()),
            Err(e) => {
                error!("获取用户列表失败:{}", e);
                Err(MessageClientError::GetFriendListFail)
            }
        }
    }
    /// 当前登录账户
    pub async fn get_account(&self) -> Result<AccountInfo, MessageClientError> {
        trace!("获取账户信息");
        let info = self.client.account_info.read().await;
        Ok(AccountInfo {
            age: info.age,
            gender: info.gender,
            nickname: info.nickname.clone(),
            uin: self.client.uin().await,
        })
    }

    async fn login(&self, state: &QRCodeConfirmed) -> Result<LoginResponse, MessageClientError> {
        match self
            .client
            .qrcode_login(&state.tmp_pwd, &state.tmp_no_pic_sig, &state.tgt_qr)
            .await
        {
            Ok(r) => Ok(r),
            Err(_) => Err(MessageClientError::LoginFail),
        }
    }
}

/// 生成登录设备
fn build_delives(device: Device) -> Device {
    let mut rng = rand::thread_rng();
    Device {
        model: "Penguin A".into(),
        brand: "Penguin".into(),
        vendor_name: "Penguin APP".into(),
        display: format!("Penguin.{}.001", rng.gen_range(100000..999999)),
        finger_print: format!(
            "penguin/iarim/sagit:10/eomam.200122.001/{}:user/release-keys",
            rng.gen_range(1000000..9999999)
        ),
        ..device
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::Level;
    use tracing_subscriber::FmtSubscriber;
    #[tokio::test]
    async fn get_login_qrcode() {
        let subscriber = FmtSubscriber::builder()
            // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
            // will be written to stdout.
            .with_max_level(Level::TRACE)
            // completes the builder.
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");
        let client = RicqClient::init().await;

        let bytes = client.get_login_qrcode().await.unwrap();

        println!("{bytes:?}");
    }
}
