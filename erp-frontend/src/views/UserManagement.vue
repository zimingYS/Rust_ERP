<template>
  <div style="padding: 20px;">
    <el-row :gutter="20" style="margin-bottom: 20px;">
      <el-col :span="8">
        <el-card shadow="hover">
          <div class="stat-card">
            <div class="stat-icon" style="background-color: #f56c6c;">
              <el-icon :size="30"><UserFilled /></el-icon>
            </div>
            <div class="stat-content">
              <div class="stat-value">{{ stats.admin_count }}</div>
              <div class="stat-label">管理员数量</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="8">
        <el-card shadow="hover">
          <div class="stat-card">
            <div class="stat-icon" style="background-color: #e6a23c;">
              <el-icon :size="30"><Avatar /></el-icon>
            </div>
            <div class="stat-content">
              <div class="stat-value">{{ stats.supervisor_count }}</div>
              <div class="stat-label">主管数量</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="8">
        <el-card shadow="hover">
          <div class="stat-card">
            <div class="stat-icon" style="background-color: #909399;">
              <el-icon :size="30"><User /></el-icon>
            </div>
            <div class="stat-content">
              <div class="stat-value">{{ stats.user_count }}</div>
              <div class="stat-label">普通用户数量</div>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <el-card style="margin-bottom: 20px;">
      <div style="display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 10px;">
        <el-form :inline="true" :model="searchForm" style="margin: 0;">
          <el-form-item label="用户名" style="margin-bottom: 0;">
            <el-input v-model="searchForm.username" placeholder="请输入用户名" clearable style="width: 200px;" />
          </el-form-item>
          <el-form-item label="角色" style="margin-bottom: 0;">
            <el-select v-model="searchForm.role_id" placeholder="请选择角色" clearable style="width: 150px;">
              <el-option
                v-for="role in roles"
                :key="role.id"
                :label="role.name"
                :value="role.id"
              />
            </el-select>
          </el-form-item>
        </el-form>
        <div style="display: flex; gap: 8px;">
          <el-button type="primary" @click="handleSearch">
            <el-icon><Search /></el-icon>
            查询
          </el-button>
          <el-button @click="handleReset">
            <el-icon><Refresh /></el-icon>
            重置
          </el-button>
          <el-button type="success" @click="openAddDialog">
            <el-icon><Plus /></el-icon>
            新增用户
          </el-button>
        </div>
      </div>
    </el-card>

    <el-table :data="users" border style="width: 100%" v-loading="loading">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="username" label="用户名" min-width="150" />
      <el-table-column prop="role_name" label="角色" width="120">
        <template #default="scope">
          <el-tag :type="getRoleTagType(scope.row.role_id)">
            {{ scope.row.role_name }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column label="操作" width="280" fixed="right">
        <template #default="scope">
          <div style="display: flex; gap: 8px;">
            <el-button size="small" @click="openEditDialog(scope.row)">
              编辑
            </el-button>
            <el-button size="small" type="warning" @click="openResetPassword(scope.row)">
              重置密码
            </el-button>
            <el-button
              size="small"
              type="danger"
              @click="deleteUser(scope.row.id)"
              :disabled="scope.row.id === 1"
            >
              删除
            </el-button>
          </div>
        </template>
      </el-table-column>
    </el-table>

    <el-dialog
      v-model="dialogVisible"
      :title="isEdit ? '编辑用户' : '新增用户'"
      width="500px"
    >
      <el-form :model="userForm" label-width="100px">
        <el-form-item label="用户名" required v-if="!isEdit">
          <el-input v-model="userForm.username" placeholder="请输入用户名" />
        </el-form-item>
        <el-form-item label="密码" required v-if="!isEdit">
          <el-input
            v-model="userForm.password"
            type="password"
            placeholder="请输入密码"
            show-password
          />
        </el-form-item>
        <el-form-item label="角色" required>
          <el-select v-model="userForm.role_id" placeholder="请选择角色" style="width: 100%;">
            <el-option
              v-for="role in roles"
              :key="role.id"
              :label="role.name"
              :value="role.id"
            />
          </el-select>
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" @click="saveUser">确认提交</el-button>
      </template>
    </el-dialog>

    <el-dialog v-model="resetPasswordVisible" title="重置密码" width="400px">
      <p>确定要重置用户 <strong>{{ currentUser?.username }}</strong> 的密码吗？</p>
      <p style="color: #f56c6c; margin-top: 10px;">重置后密码将变为：123456</p>
      <template #footer>
        <el-button @click="resetPasswordVisible = false">取消</el-button>
        <el-button type="primary" @click="confirmResetPassword">确认重置</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import request from '../utils/request'
import { ElMessage, ElMessageBox } from 'element-plus'
import { User, UserFilled, Avatar, Search, Refresh, Plus } from '@element-plus/icons-vue'

const router = useRouter()
const users = ref([])
const roles = ref([])
const loading = ref(false)
const dialogVisible = ref(false)
const resetPasswordVisible = ref(false)
const isEdit = ref(false)
const currentId = ref(null)
const currentUser = ref(null)

const stats = ref({
  admin_count: 0,
  supervisor_count: 0,
  user_count: 0
})

const searchForm = ref({
  username: '',
  role_id: null
})

const userForm = ref({
  username: '',
  password: '',
  role_id: 3
})

const getRoleTagType = (roleId) => {
  switch(roleId) {
    case 1: return 'danger'
    case 2: return 'warning'
    case 3: return 'info'
    default: return 'info'
  }
}

const fetchUsers = async () => {
  loading.value = true
  try {
    const params = {}
    if (searchForm.value.username) {
      params.username = searchForm.value.username
    }
    if (searchForm.value.role_id) {
      params.role_id = searchForm.value.role_id
    }

    users.value = await request.get('/api/users', { params })
    calculateStats()
  } catch (e) {
    console.error('获取用户数据失败', e)
    ElMessage.error('获取用户数据失败')
  } finally {
    loading.value = false
  }
}

const fetchRoles = async () => {
  try {
    const fetchedRoles = await request.get('/api/users/roles')

    if (fetchedRoles.length < 3) {
      roles.value = [
        { id: 1, name: '管理员' },
        { id: 2, name: '主管' },
        { id: 3, name: '普通用户' }
      ]
    } else {
      roles.value = fetchedRoles
    }
  } catch (e) {
    console.error('获取角色数据失败', e)
    roles.value = [
      { id: 1, name: '管理员' },
      { id: 2, name: '主管' },
      { id: 3, name: '普通用户' }
    ]
  }
}

const calculateStats = () => {
  stats.value.admin_count = users.value.filter(u => u.role_id === 1).length
  stats.value.supervisor_count = users.value.filter(u => u.role_id === 2).length
  stats.value.user_count = users.value.filter(u => u.role_id === 3).length
}

const handleSearch = () => {
  fetchUsers()
}

const handleReset = () => {
  searchForm.value = {
    username: '',
    role_id: null
  }
  fetchUsers()
}

const openAddDialog = () => {
  isEdit.value = false
  userForm.value = {
    username: '',
    password: '',
    role_id: 3
  }
  dialogVisible.value = true
}

const openEditDialog = (user) => {
  isEdit.value = true
  currentId.value = user.id
  userForm.value = {
    username: user.username,
    password: '',
    role_id: user.role_id
  }
  dialogVisible.value = true
}

const saveUser = async () => {
  if (!isEdit.value && !userForm.value.username) {
    ElMessage.warning('请输入用户名')
    return
  }

  if (!isEdit.value && !userForm.value.password) {
    ElMessage.warning('请输入密码')
    return
  }

  if (!userForm.value.role_id) {
    ElMessage.warning('请选择角色')
    return
  }

  try {
    if (isEdit.value) {
      await request.put(`/api/users/${currentId.value}`, {
        role_id: userForm.value.role_id
      })
      ElMessage.success('更新成功')
    } else {
      await request.post('/api/users', userForm.value)
      ElMessage.success('创建成功')
    }
    dialogVisible.value = false
    fetchUsers()
  } catch (error) {
    const errorMsg = error.response?.data?.error || '操作失败'
    ElMessage.error(errorMsg)
  }
}

const deleteUser = async (id) => {
  try {
    await ElMessageBox.confirm('确定要删除这个用户吗？', '警告', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    await request.delete(`/api/users/${id}`)
    ElMessage.success('删除成功')
    fetchUsers()
  } catch (error) {
    if (error !== 'cancel') {
      const errorMsg = error.response?.data?.error || '删除失败'
      ElMessage.error(errorMsg)
    }
  }
}

const openResetPassword = (user) => {
  currentUser.value = user
  resetPasswordVisible.value = true
}

const confirmResetPassword = async () => {
  try {
    await request.post(`/api/users/${currentUser.value.id}/reset-password`)
    ElMessage.success('密码已重置为 123456')
    resetPasswordVisible.value = false
  } catch (error) {
    const errorMsg = error.response?.data?.error || '重置失败'
    ElMessage.error(errorMsg)
  }
}

onMounted(() => {
  const token = localStorage.getItem('erp_token')
  if (!token) {
    router.push('/login')
  } else {
    fetchUsers()
    fetchRoles()
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
