import { createRouter, createWebHistory } from 'vue-router'
import Home from '../views/Home.vue'
import GPA from '../views/GPA.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: Home,
    },
    {
      path: '/gpa',
      name: 'GPA',
      component:GPA,
    },
  ],
})

export default router
