import { createApp } from "vue";
import AreaSelector from "./AreaSelector.vue";

if (import.meta.env.PROD) {
  // Area selector window should also hide the context menu in release
  window.addEventListener("contextmenu", (event) => event.preventDefault());
}

createApp(AreaSelector).mount("#app");
