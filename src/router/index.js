import { createRouter, createWebHistory } from "vue-router";
import AboutPage from "../pages/AboutPage.vue";
import ChartsPage from "../pages/ChartsPage.vue";
import IndexPage from "../pages/IndexPage.vue";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      name: "home",
      component: IndexPage,
    },
    {
      path: "/charts",
      name: "charts",
      component: ChartsPage,
    },
    {
      path: "/about",
      name: "about",
      component: AboutPage,
    },
  ],
});

export default router;