import { createRouter, createWebHashHistory } from "vue-router";

const routes = [
  {
    path: "/",
    component: () => import("../pages/landing.vue"),
  },
  {
    path: "/http/",
    component: () => import("../pages/HttpRequests.vue"),
  },
  {
    path: "/netio/",
    component: () => import("../pages/NetIO_Requests.vue"),
  },
  {
    path: "/settings/",
    component: () => import("../pages/SettingsPage.vue"),
  },
];

export const router = createRouter({
  history: createWebHashHistory(),
  routes,
});
