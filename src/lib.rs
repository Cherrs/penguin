#![feature(async_fn_in_trait)]

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

pub trait MessageClient {
    async fn get_login_qrcode(&self) -> Result<Bytes, MessageClientError>;
    async fn get_qrcode_state(&self) -> Result<LoginState, MessageClientError>;
    async fn get_account(&self) -> Result<AccountInfo, MessageClientError>;
    async fn get_friend_list(&self) -> Result<FriendList, MessageClientError>;
    async fn get_login_qrcode_base64(&self) -> Result<String, MessageClientError>;
}

#[derive(Error, Debug)]
pub enum MessageClientError {
    #[error("获取二维码失败")]
    FetchQrcodeFail,
    #[error("获取扫码状态失败")]
    GetQrcodeStateFail,
    #[error("登录失败")]
    LoginFail,
    #[error("获取好友列表失败")]
    GetFriendListFail,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct QrcodeConfirmed(pub i64);

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum LoginState {
    /// 等待扫码
    WaitingForScan,
    /// 已经扫码，等待确认
    WaitingForConfirm,
    /// 超时
    Timeout,
    /// 已经确认
    Confirmed(QrcodeConfirmed),
    /// 取消
    Canceled,
}

/// 好友信息
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct FriendInfo {
    pub uin: i64,
    pub nick: String,
    pub remark: String,
    pub face_id: i16,
    pub group_id: u8,
}

/// 好友分组信息
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct FriendGroupInfo {
    pub group_id: u8,
    pub group_name: String,
    pub friend_count: i32,
    pub online_friend_count: i32,
    pub seq_id: u8,
}

/// 好友列表
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct FriendList {
    /// 好友列表
    pub friends: Vec<FriendInfo>,
    /// 好友分组
    pub friend_groups: HashMap<u8, FriendGroupInfo>,
    /// 好友数量
    pub total_count: i16,
    /// 在线好友数量
    pub online_friend_count: i16,
}

/// 账户信息
#[derive(Default, Debug, Deserialize, Serialize)]
pub struct AccountInfo {
    /// 昵称
    pub nickname: String,
    /// 年龄
    pub age: u8,
    ///
    pub gender: u8,
    /// QQ号
    pub uin: i64,
}
