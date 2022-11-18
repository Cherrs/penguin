use std::str::Bytes;

use async_trait::async_trait;

#[async_trait]
pub trait MessageClient {
    /// 初始化Client
    async fn init() -> Self;
    /// 获取登录二维码
    async fn get_login_qrcode(&self) -> Bytes;
}
