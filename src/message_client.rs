use async_trait::async_trait;
use bytes::Bytes;

#[async_trait]
pub trait MessageClient {
    /// 初始化Client
    async fn init() -> Self;
    /// 获取登录二维码
    async fn get_login_qrcode(&self) -> Bytes;

    async fn get_qrcode_state(&self, sig: Bytes) -> QrcodeState;
}

pub enum QrcodeState {}
