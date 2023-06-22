import { createApp } from 'vue'
import './app.css'
import App from './App.vue'
import Main from "./Main.vue"
import Settings from "./pages/Settings.vue"
import Preview from "./pages/Preview.vue"
import { createRouter, createWebHistory } from "vue-router"


const routes = [
    { path: "/", name: "root", component: App },
    { path: "/settings", name: "settings", component: Settings },
    { path: "/preview", name: "preview", component: Preview }
]

const router = createRouter({
    history: createWebHistory(),
    routes
});


const app = createApp(Main);
app.use(router)
app.mount("#app")
