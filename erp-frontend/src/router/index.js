import { createRouter, createWebHistory } from 'vue-router'
import Login from '../views/Login.vue'
import Register from '../views/Register.vue'
import HomeDashboard from '../views/HomeDashboard.vue'
import ProductManagement from '../views/ProductManagement.vue'
import PurchaseManagement from "../views/PurchaseManagement.vue";
import SalesManagement from "../views/SalesManagement.vue";
import InventoryManagement from "../views/InventoryManagement.vue";
import FinanceManagement from "../views/FinanceManagement.vue";
import AuditLog from "../views/AuditLog.vue";
import ReportExport from "../views/ReportExport.vue";
import UserManagement from "../views/UserManagement.vue";
import SupplierManagement from "../views/SupplierManagement.vue";
import CustomerManagement from "../views/CustomerManagement.vue";
import Layout from '../components/Layout.vue'

const routes = [
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
                path: '',
                redirect: '/home'
            },
            {
                path: 'home',
                name: 'HomeDashboard',
                component: HomeDashboard
            },
            {
                path: 'products',
                name: 'ProductManagement',
                component: ProductManagement
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
                path: 'suppliers',
                name: 'SupplierManagement',
                component: SupplierManagement
            },
            {
                path: 'customers',
                name: 'CustomerManagement',
                component: CustomerManagement
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
                path: 'reports',
                name: 'ReportExport',
                component: ReportExport
            },
            {
                path: 'audit',
                name: 'AuditLog',
                component: AuditLog
            },
            {
                path: 'users',
                name: 'UserManagement',
                component: UserManagement
            }
        ]
    }
]

const router = createRouter({
    history: createWebHistory(),
    routes
})

router.beforeEach((to, from, next) => {
    const token = localStorage.getItem('erp_token')
    if (to.meta.requiresAuth && !token) {
        next('/login')
    } else {
        next()
    }
})

export default router