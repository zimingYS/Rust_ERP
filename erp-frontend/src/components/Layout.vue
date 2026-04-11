<template>
  <el-container style="height: 100vh;">
    <el-aside :width="isCollapse ? '64px' : '200px'" class="sidebar" style="background-color: #304156; transition: width 0.3s;">
      <div class="logo">
        <span v-if="!isCollapse">ERP 系统</span>
        <span v-else>ERP</span>
      </div>

      <el-menu
          :default-active="activeMenu"
          :collapse="isCollapse"
          background-color="#304156"
          text-color="#bfcbd9"
          active-text-color="#409EFF"
          router
      >
        <el-menu-item index="/dashboard">
          <el-icon><HomeFilled /></el-icon>
          <template #title>物料管理</template>
        </el-menu-item>

        <el-menu-item index="/purchase">
          <el-icon><ShoppingCart /></el-icon>
          <template #title>采购管理</template>
        </el-menu-item>

        <el-menu-item index="/sales">
          <el-icon><Sell /></el-icon>
          <template #title>销售管理</template>
        </el-menu-item>

        <el-menu-item index="/inventory">
          <el-icon><Box /></el-icon>
          <template #title>库存管理</template>
        </el-menu-item>

        <el-menu-item index="/finance">
          <el-icon><Coin /></el-icon>
          <template #title>财务管理</template>
        </el-menu-item>

        <el-menu-item index="/reports">
          <el-icon><DataAnalysis /></el-icon>
          <template #title>报表导出</template>
        </el-menu-item>

        <el-menu-item v-if="isAdmin" index="/audit">
          <el-icon><Notebook /></el-icon>
          <template #title>日志审计</template>
        </el-menu-item>

        <el-menu-item v-if="isAdmin" index="/users">
          <el-icon><User /></el-icon>
          <template #title>用户管理</template>
        </el-menu-item>
      </el-menu>
    </el-aside>

    <el-container class="main-container">
      <el-header style="background-color: #fff; border-bottom: 1px solid #dfe6ec; display: flex; align-items: center; justify-content: space-between; padding: 0 20px;">
        <div style="display: flex; align-items: center;">
          <el-icon
              @click="toggleCollapse"
              style="font-size: 20px; cursor: pointer; margin-right: 15px;"
          >
            <Fold v-if="!isCollapse" />
            <Expand v-else />
          </el-icon>
          <el-breadcrumb separator="/">
            <el-breadcrumb-item :to="{ path: '/dashboard' }">首页</el-breadcrumb-item>
            <el-breadcrumb-item>{{ pageTitle }}</el-breadcrumb-item>
          </el-breadcrumb>
        </div>

        <div style="display: flex; align-items: center; gap: 15px;">
          <el-dropdown @command="handleCommand">
            <span style="cursor: pointer; display: flex; align-items: center; gap: 5px;">
              <el-icon><User /></el-icon>
              <span>{{ username }}</span>
              <el-icon><ArrowDown /></el-icon>
            </span>
            <template #dropdown>
              <el-dropdown-menu>
                <el-dropdown-item command="logout">
                  <el-icon><SwitchButton /></el-icon>
                  退出登录
                </el-dropdown-item>
              </el-dropdown-menu>
            </template>
          </el-dropdown>
        </div>
      </el-header>

      <el-main style="background-color: #f0f2f5; padding: 20px;">
        <router-view />
      </el-main>
    </el-container>
  </el-container>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import {
  HomeFilled,
  ShoppingCart,
  Sell,
  Box,
  Coin,
  Notebook,
  DataAnalysis,
  User,
  Fold,
  Expand,
  ArrowDown,
  SwitchButton
} from '@element-plus/icons-vue'
import { ElMessageBox } from 'element-plus'

const router = useRouter()
const route = useRoute()
const isCollapse = ref(false)
const username = ref(localStorage.getItem('username') || '管理员')

const userRole = ref(parseInt(localStorage.getItem('user_role') || '2'))
const isAdmin = computed(() => userRole.value === 1)

const activeMenu = computed(() => route.path)

const pageTitle = computed(() => {
  const titles = {
    '/dashboard': '物料管理',
    '/purchase': '采购管理',
    '/sales': '销售管理',
    '/inventory': '库存管理',
    '/finance': '财务管理',
    '/audit': '日志审计',
    '/reports': '报表导出',
    '/users': '用户管理'
  }
  return titles[route.path] || 'ERP 系统'
})

const toggleCollapse = () => {
  isCollapse.value = !isCollapse.value
}

const handleCommand = (command) => {
  if (command === 'logout') {
    ElMessageBox.confirm('确定要退出登录吗？', '提示', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    }).then(() => {
      localStorage.removeItem('erp_token')
      localStorage.removeItem('user_role')
      localStorage.removeItem('username')
      router.push('/login')
    }).catch(() => {})
  }
}

onMounted(() => {
  const storedUsername = localStorage.getItem('username')
  if (storedUsername) {
    username.value = storedUsername
  }
})
</script>

<style scoped>
.sidebar {
  transition: width 0.3s;
  overflow: hidden;
}

.main-container {
  display: flex;
  flex-direction: column;
  min-width: 0;
}

.logo {
  height: 60px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
  font-size: 18px;
  font-weight: bold;
  background-color: #2b3a4b;
}

:deep(.el-menu) {
  border-right: none;
}

:deep(.el-menu-item:hover) {
  background-color: #263445 !important;
}

:deep(.el-menu-item.is-active) {
  background-color: #1f2d3d !important;
}
</style>