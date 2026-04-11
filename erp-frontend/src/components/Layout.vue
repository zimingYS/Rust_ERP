<template>
  <el-container style="height: 100vh;">
    <!-- 左侧菜单栏 -->
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

        <el-menu-item index="/audit">
          <el-icon><Notebook /></el-icon>
          <template #title>日志审计</template>
        </el-menu-item>

        <el-menu-item index="/reports">
          <el-icon><DataAnalysis /></el-icon>
          <template #title>报表导出</template>
        </el-menu-item>
      </el-menu>
    </el-aside>

    <!-- 右侧内容区 -->
    <el-container class="main-container">
      <!-- 顶部导航栏 -->
      <el-header style="background-color: #fff; border-bottom: 1px solid #dfe6ec; display: flex; align-items: center; justify-content: space-between; padding: 0 20px;">
        <div style="display: flex; align-items: center;">
          <el-icon
              @click="toggleCollapse"              style="font-size: 20px; cursor: pointer; margin-right: 15px;"
          >
            <Fold v-if="!isCollapse" />
            <Expand v-else />
          </el-icon>
          <h2 style="margin: 0;">{{ pageTitle }}</h2>
        </div>

        <div style="display: flex; align-items: center; gap: 15px;">
          <el-dropdown @command="handleCommand">
            <span style="cursor: pointer; display: flex; align-items: center; gap: 5px;">
              <el-icon><User /></el-icon>
              <span>管理员</span>
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

      <!-- 主要内容区 -->
      <el-main style="background-color: #f0f2f5; padding: 20px;">
        <router-view />
      </el-main>
    </el-container>
  </el-container>
</template>

<script setup>import { ref, computed } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import {
  HomeFilled,
  ShoppingCart,
  Sell,
  Box,
  Coin,
  Notebook,
  DataAnalysis,
  Fold,
  Expand,
  User,
  ArrowDown,
  SwitchButton
} from '@element-plus/icons-vue'
import { ElMessageBox } from 'element-plus'

const router = useRouter()
const route = useRoute()
const isCollapse = ref(false)

const activeMenu = computed(() => route.path)

const pageTitle = computed(() => {
  const titles = {
    '/dashboard': '物料管理',
    '/purchase': '采购管理',
    '/sales': '销售管理',
    '/inventory': '库存管理',
    '/finance': '财务管理',
    '/audit': '日志审计',
    '/reports': '报表导出'
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
      router.push('/login')
    }).catch(() => {})
  }
}
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

.el-menu {
  border-right: none;
}

.el-menu-item:hover {
  background-color: #263445 !important;
}

.el-menu-item.is-active {
  background-color: #1f2d3d !important;
}
</style>