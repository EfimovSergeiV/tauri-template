import { createRouter, createWebHistory } from "vue-router";
import AboutPage from "../pages/about.vue";
import IndexPage from "../pages/index.vue";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      name: "home",
      component: IndexPage,
    },
    {
      path: "/about",
      name: "about",
      component: AboutPage,
    },
  ],
});

export default router;