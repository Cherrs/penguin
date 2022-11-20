<script>
    import Password from "./Password.svelte";
    import Qrcode from "./Qrcode.svelte";

    let is_pwd = true;
</script>

<div class="tab">
    <button
        class="login {is_pwd ? '' : 'active'}"
        on:click={() => (is_pwd = true)}>登录</button
    >
    <button
        class="qrcode {is_pwd ? 'active' : ''}"
        on:click={() => (is_pwd = false)}>二维码</button
    >
</div>
<div class="main">
    {#if is_pwd}
        <Password />
    {:else}
        <Qrcode />
    {/if}
</div>

<style>
    .main {
        display: flex;
        flex-direction: column;
        align-items: center;
        margin-top: 38px;
    }
    .tab {
        display: flex;
        justify-content: center;
        /* flex-direction: row; */
        padding: 0px;
        filter: drop-shadow(0px 4px 4px rgba(0, 0, 0, 0.25));
        border-radius: 12px;
    }
    .login {
        width: 156px;
        height: 47px;
        border-radius: 12px 0px 0px 12px;
    }
    .qrcode {
        width: 156px;
        height: 47px;

        border-radius: 0px 12px 12px 0px;
    }
    .active {
        color: #128c7e;
        background: #ffffff;
    }
    button {
        font-style: normal;
        font-size: 18px;
        line-height: 23px;
        background: #128c7e;
        color: #ffffff;
    }
</style>
