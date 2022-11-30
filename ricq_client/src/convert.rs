use std::collections::HashMap;

use penguin::{FriendGroupInfo, FriendInfo, FriendList};

pub fn to_friend_list_response(
    value: ricq_core::command::friendlist::FriendListResponse,
) -> FriendList {
    FriendList {
        friends: value.friends.into_iter().map(to_friend_info).collect(),
        friend_groups: value
            .friend_groups
            .into_iter()
            .map(|f| (f.0, to_friend_group_info(f.1)))
            .collect::<HashMap<u8, FriendGroupInfo>>(),
        total_count: value.total_count,
        online_friend_count: value.online_friend_count,
    }
}

pub fn to_friend_group_info(value: ricq::structs::FriendGroupInfo) -> FriendGroupInfo {
    FriendGroupInfo {
        group_id: value.group_id,
        group_name: value.group_name,
        friend_count: value.friend_count,
        online_friend_count: value.online_friend_count,
        seq_id: value.seq_id,
    }
}

pub fn to_friend_info(value: ricq::structs::FriendInfo) -> FriendInfo {
    FriendInfo {
        face_id: value.face_id,
        group_id: value.group_id,
        nick: value.nick,
        remark: value.remark,
        uin: value.uin,
    }
}
