import { createRouter, createWebHashHistory } from "vue-router";

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: "/",
      redirect: "/invite",
    },
    {
      path: "/invite",
      name: "invite",
      component: () => import("@/views/InviteView.vue"),
    },
    
    {
      path: "/dashboard",
      name: "dashboard",
      component: () => import("@/views/DashboardView.vue"),
    },
    {
      path: "/models",
      name: "models",
      component: () => import("@/views/ModelsView.vue"),
    },
    {
      path: "/models",
      name: "models",
      component: () => import("@/views/ModelsView.vue"),
    },
    {
      path: "/teams",
      name: "teams",
      component: () => import("@/views/TeamsView.vue"),
    },
    // Nav Secondary
    {
      path: "/settings",
      name: "settings",
      component: () => import("@/views/SettingsView.vue"),
    },
    {
      path: "/playground",
      name: "playground",
      component: () => import("@/views/PlayGroundView.vue"),
    },

  ],
});

export default router;