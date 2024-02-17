import { createApp } from "vue";
import App from "./App.vue";
import { createPinia } from 'pinia';
import { Quasar } from 'quasar'
import quasarLang from 'quasar/lang/zh-CN'
import 'quasar/dist/quasar.css'

const app = createApp(App);

app.use(createPinia());
app.use(Quasar, {
    plugins: {}, // import Quasar plugins and add here
    lang: quasarLang,
})
app.mount("#app");
