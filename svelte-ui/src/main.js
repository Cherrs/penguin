import './app.css'
import App from './App.svelte'
import font from '../public/SourceHanSansHWSC-VF.otf.woff2'

document.addEventListener('contextmenu', event => event.preventDefault());

var fontFile = new FontFace('SourceHanSansCN', `url(${font})`);
let app;
fontFile.load().then(() => {
  app = new App({
    target: document.getElementById('app')
  })

});
export default app
