import { createRouter, createWebHashHistory } from "vue-router";
import LandingPage from "../pages/landing.vue";

const routes = [
  {
    path: "/",
    component: LandingPage, 
  },
  {
    path: "/http/",
    component: () => import ("../pages/HttpRequests.vue"),
  },
  {
    path: "/netio/",
    component: () => import ("../pages/NetIO_Requests.vue"),
  },
  {
    path: "/settings/",
    component: () => import ("../pages/SettingsPage.vue"),
  },
];

export const router = createRouter({
  history: createWebHashHistory(),
  routes,
});
