<template>
  <div style="padding: 20px;">
    <!-- 统计卡片 -->
    <el-row :gutter="20" style="margin-bottom: 20px;">
      <el-col :span="8">
        <el-card shadow="hover">
          <div class="stat-card">
            <div class="stat-icon" style="background-color: #409eff;">
              <el-icon :size="30"><Document /></el-icon>
            </div>
            <div class="stat-content">
              <div class="stat-value">{{ activitySummary.total_logs || 0 }}</div>
              <div class="stat-label">总日志数</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="8">
        <el-card shadow="hover">
          <div class="stat-card">
            <div class="stat-icon" style="background-color: #67c23a;">
              <el-icon :size="30"><User /></el-icon>
            </div>
            <div class="stat-content">
              <div class="stat-value">{{ Object.keys(activitySummary.user_activity || {}).length }}</div>
              <div class="stat-label">活跃用户数</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="8">
        <el-card shadow="hover">
          <div class="stat-card">
            <div class="stat-icon" style="background-color: #e6a23c;">
              <el-icon :size="30"><Menu /></el-icon>
            </div>
            <div class="stat-content">
              <div class="stat-value">{{ Object.keys(activitySummary.module_activity || {}).length }}</div>
              <div class="stat-label">活跃模块数</div>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <!-- 搜索和筛选 -->
    <el-card style="margin-bottom: 20px;">
      <el-form :inline="true" :model="searchForm">
        <el-form-item label="用户名">
          <el-input v-model="searchForm.username" placeholder="请输入用户名" clearable />
        </el-form-item>
        <el-form-item label="模块">
          <el-select v-model="searchForm.module" placeholder="请选择模块" clearable style="width: 150px;">
            <el-option label="采购管理" value="采购管理" />
            <el-option label="销售管理" value="销售管理" />
            <el-option label="库存管理" value="库存管理" />
            <el-option label="财务管理" value="财务管理" />
            <el-option label="物料管理" value="物料管理" />
            <el-option label="用户管理" value="用户管理" />
          </el-select>
        </el-form-item>
        <el-form-item label="操作">
          <el-input v-model="searchForm.action" placeholder="请输入操作" clearable />
        </el-form-item>
        <el-form-item label="日期范围">
          <el-date-picker
            v-model="searchForm.dateRange"
            type="daterange"
            range-separator="至"
            start-placeholder="开始日期"
            end-placeholder="结束日期"
            value-format="YYYY-MM-DD"
          />
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="handleSearch">查询</el-button>
          <el-button @click="handleReset">重置</el-button>
        </el-form-item>
      </el-form>
    </el-card>

    <!-- 日志表格 -->
    <el-table :data="auditLogs" border style="width: 100%" v-loading="loading">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="username" label="用户" width="120" />
      <el-table-column prop="module" label="模块" width="120" />
      <el-table-column prop="action" label="操作" width="120" />
      <el-table-column prop="method" label="请求方法" width="100">
        <template #default="scope">
          <el-tag :type="getMethodType(scope.row.method)" size="small">
            {{ scope.row.method }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column prop="path" label="路径" min-width="200" show-overflow-tooltip />
      <el-table-column prop="response_status" label="状态码" width="100">
        <template #default="scope">
          <el-tag :type="getStatusType(scope.row.response_status)" size="small">
            {{ scope.row.response_status }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column prop="ip_address" label="IP地址" width="150" />
      <el-table-column prop="description" label="描述" min-width="200" show-overflow-tooltip />
      <el-table-column prop="created_at" label="操作时间" width="180" />
    </el-table>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import request from '../utils/request'
import { ElMessage } from 'element-plus'
import { Document, User, Menu } from '@element-plus/icons-vue'

const router = useRouter()
const auditLogs = ref([])
const loading = ref(false)
const activitySummary = ref({
  total_logs: 0,
  user_activity: {},
  module_activity: {}
})

const searchForm = ref({
  username: '',
  module: '',
  action: '',
  dateRange: []
})

const fetchAuditLogs = async () => {
  loading.value = true
  try {
    const params = {}
    if (searchForm.value.username) {
      params.username = searchForm.value.username
    }
    if (searchForm.value.module) {
      params.module = searchForm.value.module
    }
    if (searchForm.value.action) {
      params.action = searchForm.value.action
    }
    if (searchForm.value.dateRange && searchForm.value.dateRange.length === 2) {
      params.start_date = searchForm.value.dateRange[0]
      params.end_date = searchForm.value.dateRange[1]
    }

    auditLogs.value = await request.get('/api/audit/logs', { params })
  } catch (e) {
    console.error('获取审计日志失败', e)
    ElMessage.error('获取审计日志失败')
  } finally {
    loading.value = false
  }
}

const fetchActivitySummary = async () => {
  try {
    activitySummary.value = await request.get('/api/audit/activity-summary')
  } catch (e) {
    console.error('获取活动摘要失败', e)
  }
}

const handleSearch = () => {
  fetchAuditLogs()
}

const handleReset = () => {
  searchForm.value = {
    username: '',
    module: '',
    action: '',
    dateRange: []
  }
  fetchAuditLogs()
}

const getMethodType = (method) => {
  switch (method) {
    case 'GET':
      return 'info'
    case 'POST':
      return 'success'
    case 'PUT':
      return 'warning'
    case 'DELETE':
      return 'danger'
    default:
      return 'info'
  }
}

const getStatusType = (status) => {
  if (status >= 200 && status < 300) return 'success'
  if (status >= 400 && status < 500) return 'warning'
  if (status >= 500) return 'danger'
  return 'info'
}

onMounted(() => {
  const token = localStorage.getItem('erp_token')
  if (!token) {
    router.push('/login')
  } else {
    fetchAuditLogs()
    fetchActivitySummary()
  }
})
</script>

<style scoped>
.stat-card {
  display: flex;
  align-items: center;
  gap: 15px;
}

.stat-icon {
  width: 60px;
  height: 60px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
}

.stat-content {
  flex: 1;
}

.stat-value {
  font-size: 24px;
  font-weight: bold;
  color: #303133;
  margin-bottom: 5px;
}

.stat-label {
  font-size: 14px;
  color: #909399;
}
</style>
