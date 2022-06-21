import { createApp } from 'vue'
import App from './App.vue'
import MdEditorV3 from 'md-editor-v3';
import 'md-editor-v3/lib/style.css';

createApp(App)
.use(MdEditorV3)
.mount('#app')
