mod ricq_client;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MessageClientError {
    #[error("获取二维码失败")]
    FetchQrcodeFail,
    #[error("获取扫码状态失败")]
    GetQrcodeStateFail,
}
