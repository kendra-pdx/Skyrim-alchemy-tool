import { createRouter, createWebHistory } from 'vue-router'
import AlchemyView from '../views/AlchemyView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: AlchemyView
    },
  ]
})

export default router
