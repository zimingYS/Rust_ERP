<template>
  <div style="padding: 20px;">
    <div v-if="!isLoggedIn" style="max-width: 400px; margin: 100px auto; border: 1px solid #eee; padding: 30px; border-radius: 8px;">
      <h2 style="text-align: center;">ERP 系统登录</h2>
      <el-form :model="loginForm" label-width="60px">
        <el-form-item label="账号">
          <el-input v-model="loginForm.username" placeholder="请输入账号" />
        </el-form-item>
        <el-form-item label="密码">
          <el-input v-model="loginForm.password" type="password" placeholder="请输入密码" show-password />
        </el-form-item>
        <el-button type="primary" style="width: 100%" @click="handleLogin" :loading="loading">登录</el-button>
      </el-form>
    </div>

    <div v-else>
      <div style="display: flex; justify-content: space-between; align-items: center;">
        <h1>ERP 物料管理系统</h1>
        <el-button @click="handleLogout">退出登录</el-button>
      </div>

      <el-button type="primary" @click="openAddDialog" style="margin-bottom: 20px;">新增物料</el-button>

      <el-table :data="items" border style="width: 100%">
        <el-table-column prop="id" label="ID" width="80" />
        <el-table-column prop="name" label="物料名称" />
        <el-table-column prop="sku" label="SKU 编号" />
        <el-table-column prop="stock" label="当前库存" />
        <el-table-column label="操作" width="180">
          <template #default="scope">
            <div style="display: flex; gap: 8px;">
              <el-button size="small" @click="handleEdit(scope.row)">编辑</el-button>
              <el-button type="danger" size="small" @click="deleteItem(scope.row.id)">删除</el-button>
            </div>
          </template>
        </el-table-column>
      </el-table>

      <el-dialog v-model="dialogVisible" :title="isEdit ? '编辑物料' : '添加新物料'" width="30%">
        <el-form :model="newItem">
          <el-form-item label="名称"><el-input v-model="newItem.name" /></el-form-item>
          <el-form-item label="SKU"><el-input v-model="newItem.sku" /></el-form-item>
          <el-form-item label="库存"><el-input-number v-model="newItem.stock" :min="0" /></el-form-item>
        </el-form>
        <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" @click="saveItem">确认提交</el-button>
</template>
</el-dialog>
</div>
</div>
</template>

<script setup>
import { ref, onMounted } from "vue"
import axios from 'axios'
import { ElMessage } from 'element-plus'

// --- 登录相关状态 ---
const isLoggedIn = ref(false)
const loading = ref(false)
const loginForm = ref({ username: '', password: '' })

// --- 业务相关状态 ---
const items = ref([])
const dialogVisible = ref(false)
const newItem = ref({ name: '', sku: '', stock: 0 })
const isEdit = ref(false)
const currentId = ref(null)

const API_BASE = 'http://127.0.0.1:3000/api'

// --- Axios 配置 (关键点) ---
// 设置请求拦截器：每次请求自动带上 Token
axios.interceptors.request.use(config => {
  const token = localStorage.getItem('erp_token')
  if (token) {
    config.headers.Authorization = `Bearer ${token}`
  }
  return config
})

// 设置响应拦截器：如果后端返回 401，说明 Token 失效，强制退回登录页
axios.interceptors.response.use(
    response => response,
    error => {
      if (error.response && error.response.status === 401) {
        ElMessage.error('登录失效，请重新登录')
        handleLogout()
      }
      return Promise.reject(error)
    }
)

// --- 登录逻辑 ---
const handleLogin = async () => {
  if (!loginForm.value.username || !loginForm.value.password) {
    return ElMessage.warning('请输入完整信息')
  }
  loading.value = true
  try {
    const res = await axios.post(`${API_BASE}/login`, loginForm.value)
    localStorage.setItem('erp_token', res.data.token) // 存储 Token
    isLoggedIn.value = true
    ElMessage.success('登录成功')
    fetchItems()
  } catch (error) {
    ElMessage.error('用户名或密码错误')
  } finally {
    loading.value = false
  }
}

const handleLogout = () => {
  localStorage.removeItem('erp_token')
  isLoggedIn.value = false
  items.value = []
}

// --- 业务逻辑 (URL 统一使用 API_BASE) ---
const fetchItems = async () => {
  try {
    const res = await axios.get(`${API_BASE}/items`)
    items.value = res.data
  } catch (e) { /* 拦截器会处理 401 */ }
}

const saveItem = async () => {
  try {
    if (isEdit.value) {
      await axios.put(`${API_BASE}/items/${currentId.value}`, newItem.value)
      ElMessage.success('更新成功!')
    } else {
      await axios.post(`${API_BASE}/items`, newItem.value)
      ElMessage.success('添加成功!')
    }
    dialogVisible.value = false
    fetchItems()
  } catch (error) {
    ElMessage.error('操作失败')
  }
}

const deleteItem = async (id) => {
  try {
    await axios.delete(`${API_BASE}/items/${id}`)
    ElMessage.success('物料已成功删除')
    fetchItems()
  } catch (error) {
    ElMessage.error('删除失败')
  }
}

const handleEdit = (row) => {
  isEdit.value = true
  currentId.value = row.id
  newItem.value = { ...row }
  dialogVisible.value = true
}

const openAddDialog = () => {
  isEdit.value = false
  newItem.value = { name: '', sku: '', stock: 0 }
  dialogVisible.value = true
}

onMounted(() => {
  const token = localStorage.getItem('erp_token')
  if (token) {
    isLoggedIn.value = true
    fetchItems()
  }
})
</script>