<script>
    import { invoke } from "@tauri-apps/api/tauri";

    let img = invoke("get_qrcode");
</script>

{#await img}
    <div class="qrcode">正在加载...</div>
{:then data}
    <div class="qrcode">
        <img src={data} style="height: 100%; width:100%" alt="" />
    </div>
{:catch err}
    <p style="color: red">{err.message}</p>
{/await}

<div class="p">请扫描屏幕上的二维码</div>
<div class="tips">
    <div><span class="i">1</span>打开手机QQ</div>
    <div><span class="i">2</span>点击右上角+</div>
    <div><span class="i">3</span>扫一扫</div>
</div>

<style>
    .qrcode {
        display: flex;
        height: 266px;
        width: 266px;
        justify-content: center;
        align-items: center;
    }
    .p {
        font-weight: bold;
        margin-top: 40px;
    }
    .i {
        display: inline-flex;
        border-radius: 50%;
        height: 24px;
        width: 24px;
        background-color: #dffaf7;
        justify-content: center;
        align-items: center;
        margin-right: 12px;
        color: #128c7e;
        font-family: "Plus Jakarta Sans";
        font-style: normal;
        font-size: 12px;
        line-height: 15px;
        font-weight: 500;
    }
    .tips {
        display: flex;
        flex-direction: column;
        font-style: normal;
        font-size: 14px;
        line-height: 24px;
        gap: 12px;
        margin-top: 24px;
    }
</style>
