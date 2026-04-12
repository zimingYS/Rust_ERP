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
        <el-menu-item index="/home">
          <el-icon><DataLine /></el-icon>
          <template #title>数据概览</template>
        </el-menu-item>

        <el-menu-item index="/products">
          <el-icon><Box /></el-icon>
          <template #title>产品管理</template>
        </el-menu-item>

        <el-menu-item index="/purchase">
          <el-icon><ShoppingCart /></el-icon>
          <template #title>采购管理</template>
        </el-menu-item>

        <el-menu-item index="/sales">
          <el-icon><Sell /></el-icon>
          <template #title>销售管理</template>
        </el-menu-item>

        <el-menu-item index="/suppliers">
          <el-icon><User /></el-icon>
          <template #title>供应商管理</template>
        </el-menu-item>

        <el-menu-item index="/customers">
          <el-icon><Avatar /></el-icon>
          <template #title>客户管理</template>
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
          <!-- 消息通知 -->
          <el-badge :value="unreadCount" :hidden="unreadCount === 0" :max="99">
            <el-icon
              @click="openNotificationDialog"
              style="font-size: 20px; cursor: pointer; color: #606266;"
            >
              <Bell />
            </el-icon>
          </el-badge>

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

      <!-- 消息通知对话框 -->
      <el-dialog v-model="notificationDialogVisible" title="消息通知" width="700px" :close-on-click-modal="false">
        <div class="notification-header">
          <el-tabs v-model="activeTab" @tab-change="handleTabChange">
            <el-tab-pane label="全部" name="all">
              <span class="tab-badge">{{ stats.total_count }}</span>
            </el-tab-pane>
            <el-tab-pane label="未读" name="unread">
              <span class="tab-badge unread">{{ stats.unread_count }}</span>
            </el-tab-pane>
          </el-tabs>
          <el-button
            v-if="stats.unread_count > 0"
            type="primary"
            size="small"
            @click="markAllAsRead"
            link
          >
            全部标记为已读
          </el-button>
        </div>

        <el-scrollbar height="500px">
          <div v-if="notifications.length === 0" class="empty-notification">
            <el-empty description="暂无消息" />
          </div>
          <div v-else class="notification-list">
            <div
              v-for="notif in notifications"
              :key="notif.id"
              class="notification-item"
              :class="{ 'unread': !notif.is_read }"
              @click="handleNotificationClick(notif)"
            >
              <div class="notification-icon">
                <el-icon :size="24" :color="getNotificationColor(notif)">
                  <component :is="getNotificationIcon(notif)" />
                </el-icon>
              </div>
              <div class="notification-content">
                <div class="notification-title">
                  {{ notif.title }}
                  <el-tag v-if="!notif.is_read" size="small" type="danger" effect="dark">未读</el-tag>
                  <el-tag v-if="notif.priority === '高'" size="small" type="danger">重要</el-tag>
                </div>
                <div class="notification-text">{{ notif.content }}</div>
                <div class="notification-time">{{ formatTime(notif.created_at) }}</div>
              </div>
              <div class="notification-actions">
                <el-button
                  v-if="!notif.is_read"
                  size="small"
                  type="primary"
                  link
                  @click.stop="markAsRead(notif.id)"
                >
                  标记已读
                </el-button>
                <el-button
                  size="small"
                  type="danger"
                  link
                  @click.stop="deleteNotification(notif.id)"
                >
                  删除
                </el-button>
              </div>
            </div>
          </div>
        </el-scrollbar>
      </el-dialog>
    </el-container>
  </el-container>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import {
  DataLine,
  Box,
  ShoppingCart,
  Sell,
  Coin,
  Notebook,
  DataAnalysis,
  User,
  Avatar,
  Fold,
  Expand,
  ArrowDown,
  SwitchButton
} from '@element-plus/icons-vue'
import { ElMessageBox } from 'element-plus'

const router = useRouter()
const route = useRoute()
const isCollapse = ref(false)
const username = ref(localStorage.getItem('username') || '用户')
const notificationDialogVisible = ref(false)
const activeTab = ref('all')
const notifications = ref([])
const unreadCount = ref(0)
const stats = ref({
  total_count: 0,
  unread_count: 0,
  high_priority_count: 0
})

let pollingTimer = null

const userRole = ref(parseInt(localStorage.getItem('user_role') || '3'))
const isAdmin = computed(() => userRole.value === 1)
const isSupervisor = computed(() => userRole.value === 2)
const hasApprovePermission = computed(() => userRole.value === 1 || userRole.value === 2)

const activeMenu = computed(() => route.path)

const pageTitle = computed(() => {
  const titles = {
    '/home': '数据概览',
    '/products': '产品管理',
    '/purchase': '采购管理',
    '/sales': '销售管理',
    '/suppliers': '供应商管理',
    '/customers': '客户管理',
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

const openNotificationDialog = () => {
  notificationDialogVisible.value = true
  fetchNotifications()
}

const handleTabChange = (tab) => {
  fetchNotifications()
}

const fetchNotifications = async () => {
  try {
    const params = {}
    if (activeTab.value === 'unread') {
      params.is_read = false
    }
    notifications.value = await request.get('/api/notifications', { params })
  } catch (e) {
    console.error('获取消息列表失败', e)
  }
}

const fetchUnreadCount = async () => {
  try {
    unreadCount.value = await request.get('/api/notifications/unread-count')
  } catch (e) {
    console.error('获取未读数失败', e)
  }
}

const fetchStats = async () => {
  try {
    stats.value = await request.get('/api/notifications/stats')
  } catch (e) {
    console.error('获取统计数据失败', e)
  }
}

const markAsRead = async (id) => {
  try {
    await request.post(`/api/notifications/${id}/read`)
    await refreshNotifications()
  } catch (error) {
    console.error('标记已读失败', error)
  }
}

const markAllAsRead = async () => {
  try {
    await request.post('/api/notifications/mark-all-read')
    await refreshNotifications()
  } catch (error) {
    console.error('全部标记已读失败', error)
  }
}

const deleteNotification = async (id) => {
  try {
    await request.delete(`/api/notifications/${id}`)
    await refreshNotifications()
  } catch (error) {
    console.error('删除消息失败', error)
  }
}

const handleNotificationClick = async (notif) => {
  if (!notif.is_read) {
    await markAsRead(notif.id)
  }

  // 如果有相关链接，可以跳转
  if (notif.related_type && notif.related_id) {
    notificationDialogVisible.value = false
    switch(notif.related_type) {
      case 'purchase_order':
        router.push(`/purchase?id=${notif.related_id}`)
        break
      case 'sales_order':
        router.push(`/sales?id=${notif.related_id}`)
        break
      case 'inventory':
        router.push(`/inventory?id=${notif.related_id}`)
        break
    }
  }
}

const refreshNotifications = async () => {
  await fetchNotifications()
  await fetchUnreadCount()
  await fetchStats()
}

const getNotificationIcon = (notif) => {
  switch(notif.notification_type) {
    case '库存预警':
      return Warning
    case '订单审批':
      return CircleCheck
    case '系统通知':
      return InfoFilled
    case '任务完成':
      return SuccessFilled
    default:
      return Bell
  }
}

const getNotificationColor = (notif) => {
  switch(notif.notification_type) {
    case '库存预警':
      return '#f56c6c'
    case '订单审批':
      return '#67c23a'
    case '系统通知':
      return '#409eff'
    case '任务完成':
      return '#e6a23c'
    default:
      return '#909399'
  }
}

const formatTime = (timeStr) => {
  if (!timeStr) return ''
  const date = new Date(timeStr)
  const now = new Date()
  const diff = now - date

  // 小于1分钟
  if (diff < 60000) {
    return '刚刚'
  }
  // 小于1小时
  if (diff < 3600000) {
    return `${Math.floor(diff / 60000)}分钟前`
  }
  // 小于24小时
  if (diff < 86400000) {
    return `${Math.floor(diff / 3600000)}小时前`
  }
  // 小于7天
  if (diff < 604800000) {
    return `${Math.floor(diff / 86400000)}天前`
  }

  return date.toLocaleDateString('zh-CN')
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
      if (pollingTimer) {
        clearInterval(pollingTimer)
      }
      router.push('/login')
    }).catch(() => {})
  }
}

onMounted(() => {
  const storedUsername = localStorage.getItem('username')
  if (storedUsername) {
    username.value = storedUsername
  }

  const token = localStorage.getItem('erp_token')
  if (token) {
    fetchUnreadCount()
    fetchStats()

    // 每30秒轮询一次未读消息数
    pollingTimer = setInterval(() => {
      fetchUnreadCount()
    }, 30000)
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

.notification-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 15px;
  padding-bottom: 10px;
  border-bottom: 1px solid #ebeef5;
}

.tab-badge {
  margin-left: 5px;
  font-size: 12px;
  color: #909399;
}

.tab-badge.unread {
  color: #f56c6c;
  font-weight: bold;
}

.empty-notification {
  padding: 40px 0;
}

.notification-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.notification-item {
  display: flex;
  gap: 12px;
  padding: 12px;
  border-radius: 8px;
  background-color: #fff;
  border: 1px solid #ebeef5;
  cursor: pointer;
  transition: all 0.3s;
}

.notification-item:hover {
  background-color: #f5f7fa;
  border-color: #dcdfe6;
}

.notification-item.unread {
  background-color: #ecf5ff;
  border-color: #b3d8ff;
}

.notification-icon {
  flex-shrink: 0;
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  background-color: #f5f7fa;
}

.notification-content {
  flex: 1;
  min-width: 0;
}

.notification-title {
  display: flex;
  align-items: center;
  gap: 8px;
  font-weight: 600;
  color: #303133;
  margin-bottom: 5px;
  font-size: 14px;
}

.notification-text {
  color: #606266;
  font-size: 13px;
  line-height: 1.5;
  margin-bottom: 5px;
  overflow: hidden;
  text-overflow: ellipsis;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
}

.notification-time {
  color: #909399;
  font-size: 12px;
}

.notification-actions {
  display: flex;
  flex-direction: column;
  gap: 5px;
  flex-shrink: 0;
}
</style>