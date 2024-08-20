import { createApp } from "vue";

// Vuetify
import "vuetify/styles";
import { createVuetify } from "vuetify";
import * as components from "vuetify/components";
import * as directives from "vuetify/directives";

// ApexCharts
import VueApexCharts from "vue3-apexcharts";

// Toast Notification
import Toast from "vue-toastification";
import "vue-toastification/dist/index.css";

// Oh, Vue Icons
import { OhVueIcon, addIcons } from "oh-vue-icons";
import { BiGearFill, BiXCircleFill } from "oh-vue-icons/icons"; // Import specific icons you need

// Importing all icons (not recommended for production due to bundle size)
// import * as allIcons from "oh-vue-icons/icons";

// Adding the icons
addIcons(BiGearFill, BiXCircleFill);

// If you imported all icons, you would add them like this:
// addIcons(...Object.values(allIcons));

import "./styles.css";
import App from "./App.vue";

const vuetify = createVuetify({
  components,
  directives,
});

const app = createApp(App);

app.use(Toast);
app.use(vuetify);
app.use(VueApexCharts);
app.component("v-icon", OhVueIcon); // Register the OhVueIcon component globally

app.mount("#app");
