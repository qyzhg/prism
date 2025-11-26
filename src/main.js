import { createApp } from "vue";
import App from "./App.vue";

if (import.meta.env.PROD) {
  // Hide reload/inspect menu in release builds
  window.addEventListener("contextmenu", (event) => event.preventDefault());
}

createApp(App).mount("#app");
