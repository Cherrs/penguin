<script>
    import { invoke } from "@tauri-apps/api/tauri";
    let loading = "加载中...";
    let nick = loading;
    let uin = loading;
    let online_count = loading;
    invoke("get_account").then((x) => {
        nick = x.nickname;
        uin = x.uin;
    });
    invoke("get_friend_list").then((x) => {
        online_count = x.online_friend_count;
    });
</script>

<p>昵称：{nick}</p>
<p>QQ号：{uin}</p>
<p>在线好友数：{online_count}</p>
