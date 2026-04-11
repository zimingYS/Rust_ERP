import { createRouter, createWebHistory } from 'vue-router'
import Login from '../views/Login.vue'
import Register from '../views/Register.vue'
import Dashboard from '../views/Dashboard.vue'
import PurchaseManagement from "../views/PurchaseManagement.vue";
import SalesManagement from "../views/SalesManagement.vue";
import InventoryManagement from "../views/InventoryManagement.vue";
import FinanceManagement from "../views/FinanceManagement.vue";
import AuditLog from "../views/AuditLog.vue";
import Layout from '../components/Layout.vue'

const routes = [
    {
        path: '/',
        redirect: '/login'
    },
    {
        path: '/login',
        name: 'Login',
        component: Login
    },
    {
        path: '/register',
        name: 'Register',
        component: Register
    },
    {
        path: '/',
        component: Layout,
        meta: { requiresAuth: true },
        children: [
            {
                path: 'dashboard',
                name: 'Dashboard',
                component: Dashboard
            },
            {
                path: 'purchase',
                name: 'PurchaseManagement',
                component: PurchaseManagement
            },
            {
                path: 'sales',
                name: 'SalesManagement',
                component: SalesManagement
            },
            {
                path: 'inventory',
                name: 'InventoryManagement',
                component: InventoryManagement
            },
            {
                path: 'finance',
                name: 'FinanceManagement',
                component: FinanceManagement
            },
            {
                path: 'audit',
                name: 'AuditLog',
                component: AuditLog
            }
        ]
    }
]

const router = createRouter({
    history: createWebHistory(),
    routes
})

// 路由守卫
router.beforeEach((to, from, next) => {
    const token = localStorage.getItem('erp_token')

    if (to.meta.requiresAuth && !token) {
        next('/login')
    } else if ((to.path === '/login' || to.path === '/register') && token) {
        next('/dashboard')
    } else {
        next()
    }
})

export default router