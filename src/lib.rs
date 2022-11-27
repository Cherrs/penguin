pub mod ricq_client;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

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

impl From<ricq::structs::FriendInfo> for FriendInfo {
    fn from(value: ricq::structs::FriendInfo) -> Self {
        FriendInfo {
            face_id: value.face_id,
            group_id: value.group_id,
            nick: value.nick,
            remark: value.remark,
            uin: value.uin,
        }
    }
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

impl From<ricq::structs::FriendGroupInfo> for FriendGroupInfo {
    fn from(value: ricq::structs::FriendGroupInfo) -> Self {
        FriendGroupInfo {
            group_id: value.group_id,
            group_name: value.group_name,
            friend_count: value.friend_count,
            online_friend_count: value.online_friend_count,
            seq_id: value.seq_id,
        }
    }
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

impl From<ricq_core::command::friendlist::FriendListResponse> for FriendList {
    fn from(value: ricq_core::command::friendlist::FriendListResponse) -> Self {
        FriendList {
            friends: value.friends.into_iter().map(|f| f.into()).collect(),
            friend_groups: value
                .friend_groups
                .into_iter()
                .map(|f| (f.0, FriendGroupInfo::from(f.1)))
                .collect::<HashMap<u8, FriendGroupInfo>>(),
            total_count: value.total_count,
            online_friend_count: value.online_friend_count,
        }
    }
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
