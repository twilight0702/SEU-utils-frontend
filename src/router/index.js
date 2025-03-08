import { createRouter, createWebHistory } from 'vue-router'
import Home from '../views/Home.vue'
import GPA from '../views/GPA.vue'
import Game_2048 from '../views/game-2048.vue'

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
    {
      path: '/game-2048',
      name: 'game-2048',
      component:Game_2048,
    },
  ],
})

export default router
