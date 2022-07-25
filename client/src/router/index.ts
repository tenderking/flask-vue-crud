import { createRouter, createWebHistory } from "vue-router";
import Books from "../components/Books.vue";

const router = createRouter({
	history: createWebHistory(import.meta.env.BASE_URL),
	routes: [
		{
			path: "/",
			name: "Books",
			component: Books,
		},
		{
			path: "/ping",
			name: "ping",
			component: () => import("../components/Ping.vue"),
		},
	],
});

export default router;
