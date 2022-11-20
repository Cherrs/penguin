import './app.css'
import App from './App.svelte'

document.addEventListener('contextmenu', event => event.preventDefault());

var fontFile = new FontFace('SourceHanSansCN', 'url(../public/SourceHanSansHWSC-VF.otf.woff2)');

await fontFile.load();
const app = new App({
  target: document.getElementById('app')
})

export default app
