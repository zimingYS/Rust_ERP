<template>
  <div style="padding: 20px;">
    <!-- 统计卡片 -->
    <el-row :gutter="20" style="margin-bottom: 20px;">
      <el-col :span="6">
        <el-card shadow="hover">
          <div class="stat-card">
            <div class="stat-icon" style="background-color: #409eff;">
              <el-icon :size="30"><User /></el-icon>
            </div>
            <div class="stat-content">
              <div class="stat-value">{{ stats.total_suppliers }}</div>
              <div class="stat-label">供应商总数</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover">
          <div class="stat-card">
            <div class="stat-icon" style="background-color: #67c23a;">
              <el-icon :size="30"><CircleCheck /></el-icon>
            </div>
            <div class="stat-content">
              <div class="stat-value">{{ stats.active_suppliers }}</div>
              <div class="stat-label">活跃供应商</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover">
          <div class="stat-card">
            <div class="stat-icon" style="background-color: #909399;">
              <el-icon :size="30"><CircleClose /></el-icon>
            </div>
            <div class="stat-content">
              <div class="stat-value">{{ stats.inactive_suppliers }}</div>
              <div class="stat-label">停用供应商</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover">
          <div class="stat-card">
            <div class="stat-icon" style="background-color: #e6a23c;">
              <el-icon :size="30"><Star /></el-icon>
            </div>
            <div class="stat-content">
              <div class="stat-value">{{ stats.avg_rating.toFixed(1) }}</div>
              <div class="stat-label">平均评级</div>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <!-- 搜索和筛选 -->
    <el-card style="margin-bottom: 20px;">
      <div style="display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 10px;">
        <el-form :inline="true" :model="searchForm" style="margin: 0;">
          <el-form-item label="关键字" style="margin-bottom: 0;">
            <el-input v-model="searchForm.keyword" placeholder="供应商名称/联系人/电话" clearable style="width: 220px;" />
          </el-form-item>
          <el-form-item label="状态" style="margin-bottom: 0;">
            <el-select v-model="searchForm.status" placeholder="请选择状态" clearable style="width: 150px;">
              <el-option label="启用" value="启用" />
              <el-option label="停用" value="停用" />
            </el-select>
          </el-form-item>
          <el-form-item label="评级" style="margin-bottom: 0;">
            <el-select v-model="searchForm.rating" placeholder="请选择评级" clearable style="width: 120px;">
              <el-option label="5星" :value="5" />
              <el-option label="4星" :value="4" />
              <el-option label="3星" :value="3" />
              <el-option label="2星" :value="2" />
              <el-option label="1星" :value="1" />
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
          <el-button type="primary" @click="openAddDialog">
            <el-icon><Plus /></el-icon>
            新增供应商
          </el-button>
        </div>
      </div>
    </el-card>

    <!-- 供应商表格 -->
    <el-table :data="suppliers" border style="width: 100%" v-loading="loading">
      <el-table-column prop="id" label="ID" width="50" />
      <el-table-column prop="supplier_code" label="供应商编号" width="180" />
      <el-table-column prop="name" label="供应商名称" min-width="150" />
      <el-table-column prop="contact_person" label="联系人" width="120" />
      <el-table-column prop="phone" label="联系电话" width="130" />
      <el-table-column prop="email" label="邮箱" width="180" show-overflow-tooltip />
      <el-table-column prop="rating" label="评级" width="180" align="center">
        <template #default="scope">
          <el-rate v-model="scope.row.rating" disabled show-score text-color="#ff9900" />
        </template>
      </el-table-column>
      <el-table-column prop="status" label="状态" width="70" align="center">
        <template #default="scope">
          <el-tag :type="scope.row.status === '启用' ? 'success' : 'info'">
            {{ scope.row.status }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column label="操作" width="210">
        <template #default="scope">
          <div style="display: flex; gap: 6px; justify-content: flex-start;">
            <el-button size="small" @click="handleView(scope.row)">查看</el-button>
            <el-button size="small" @click="handleEdit(scope.row)">编辑</el-button>
            <el-button type="danger" size="small" @click="deleteSupplier(scope.row.id)">删除</el-button>
          </div>
        </template>
      </el-table-column>
    </el-table>

    <!-- 新增/编辑对话框 -->
    <el-dialog v-model="dialogVisible" :title="isEdit ? '编辑供应商' : '新增供应商'" width="700px">
      <el-form :model="supplierForm" label-width="120px">
        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item label="供应商名称" required>
              <el-input v-model="supplierForm.name" placeholder="请输入供应商名称" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="联系人">
              <el-input v-model="supplierForm.contact_person" placeholder="请输入联系人" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item label="联系电话">
              <el-input v-model="supplierForm.phone" placeholder="请输入联系电话" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="邮箱">
              <el-input v-model="supplierForm.email" placeholder="请输入邮箱" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-form-item label="地址">
          <el-input v-model="supplierForm.address" type="textarea" :rows="2" placeholder="请输入地址" />
        </el-form-item>
        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item label="税号">
              <el-input v-model="supplierForm.tax_number" placeholder="请输入税号" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="银行账号">
              <el-input v-model="supplierForm.bank_account" placeholder="请输入银行账号" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item label="付款条件">
              <el-input v-model="supplierForm.payment_terms" placeholder="如：月结30天" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="信用额度">
              <el-input-number v-model="supplierForm.credit_limit" :min="0" :precision="2" style="width: 100%;" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item label="评级">
              <el-rate v-model="supplierForm.rating" />
            </el-form-item>
          </el-col>
          <el-col :span="12" v-if="isEdit">
            <el-form-item label="状态">
              <el-radio-group v-model="supplierForm.status">
                <el-radio label="启用">启用</el-radio>
                <el-radio label="停用">停用</el-radio>
              </el-radio-group>
            </el-form-item>
          </el-col>
        </el-row>
        <el-form-item label="备注">
          <el-input v-model="supplierForm.remark" type="textarea" :rows="3" placeholder="请输入备注" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" @click="saveSupplier">确认提交</el-button>
      </template>
    </el-dialog>

    <!-- 详情对话框 -->
    <el-dialog v-model="viewDialogVisible" title="供应商详情" width="700px">
      <div v-if="currentSupplier">
        <el-descriptions :column="2" border>
          <el-descriptions-item label="供应商编号">{{ currentSupplier.supplier_code }}</el-descriptions-item>
          <el-descriptions-item label="供应商名称">{{ currentSupplier.name }}</el-descriptions-item>
          <el-descriptions-item label="联系人">{{ currentSupplier.contact_person || '-' }}</el-descriptions-item>
          <el-descriptions-item label="联系电话">{{ currentSupplier.phone || '-' }}</el-descriptions-item>
          <el-descriptions-item label="邮箱">{{ currentSupplier.email || '-' }}</el-descriptions-item>
          <el-descriptions-item label="评级">
            <el-rate v-model="currentSupplier.rating" disabled show-score text-color="#ff9900" />
          </el-descriptions-item>
          <el-descriptions-item label="状态">
            <el-tag :type="currentSupplier.status === '启用' ? 'success' : 'info'">
              {{ currentSupplier.status }}
            </el-tag>
          </el-descriptions-item>
          <el-descriptions-item label="信用额度">
            ¥{{ formatCreditLimit(currentSupplier.credit_limit) }}
          </el-descriptions-item>
          <el-descriptions-item label="地址" :span="2">{{ currentSupplier.address || '-' }}</el-descriptions-item>
          <el-descriptions-item label="税号">{{ currentSupplier.tax_number || '-' }}</el-descriptions-item>
          <el-descriptions-item label="银行账号">{{ currentSupplier.bank_account || '-' }}</el-descriptions-item>
          <el-descriptions-item label="付款条件">{{ currentSupplier.payment_terms || '-' }}</el-descriptions-item>
          <el-descriptions-item label="备注" :span="2">{{ currentSupplier.remark || '-' }}</el-descriptions-item>
        </el-descriptions>
      </div>
      <template #footer>
        <el-button @click="viewDialogVisible = false">关闭</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import request from '../utils/request'
import { ElMessage, ElMessageBox } from 'element-plus'
import { User, CircleCheck, CircleClose, Star, Search, Refresh, Plus } from '@element-plus/icons-vue'

const router = useRouter()
const suppliers = ref([])
const loading = ref(false)
const dialogVisible = ref(false)
const viewDialogVisible = ref(false)
const isEdit = ref(false)
const currentId = ref(null)
const currentSupplier = ref(null)

const stats = ref({
  total_suppliers: 0,
  active_suppliers: 0,
  inactive_suppliers: 0,
  avg_rating: 0
})

const searchForm = ref({
  keyword: '',
  status: '',
  rating: null
})

const supplierForm = ref({
  name: '',
  contact_person: '',
  phone: '',
  email: '',
  address: '',
  tax_number: '',
  bank_account: '',
  payment_terms: '',
  credit_limit: 0,
  rating: 3,
  status: '启用',
  remark: ''
})

const fetchSuppliers = async () => {
  loading.value = true
  try {
    const params = {}
    if (searchForm.value.keyword) {
      params.keyword = searchForm.value.keyword
    }
    if (searchForm.value.status) {
      params.status = searchForm.value.status
    }
    if (searchForm.value.rating) {
      params.rating = searchForm.value.rating
    }

    suppliers.value = await request.get('/api/suppliers', { params })
  } catch (e) {
    console.error('获取供应商列表失败', e)
    ElMessage.error('获取供应商列表失败')
  } finally {
    loading.value = false
  }
}

const fetchStats = async () => {
  try {
    stats.value = await request.get('/api/suppliers/stats')
  } catch (e) {
    console.error('获取统计数据失败', e)
  }
}

const handleSearch = () => {
  fetchSuppliers()
}

const handleReset = () => {
  searchForm.value = {
    keyword: '',
    status: '',
    rating: null
  }
  fetchSuppliers()
}

const openAddDialog = () => {
  isEdit.value = false
  currentId.value = null
  supplierForm.value = {
    name: '',
    contact_person: '',
    phone: '',
    email: '',
    address: '',
    tax_number: '',
    bank_account: '',
    payment_terms: '',
    credit_limit: 0,
    rating: 3,
    status: '启用',
    remark: ''
  }
  dialogVisible.value = true
}

const handleEdit = (row) => {
  isEdit.value = true
  currentId.value = row.id
  supplierForm.value = {
    name: row.name,
    contact_person: row.contact_person || '',
    phone: row.phone || '',
    email: row.email || '',
    address: row.address || '',
    tax_number: row.tax_number || '',
    bank_account: row.bank_account || '',
    payment_terms: row.payment_terms || '',
    credit_limit: row.credit_limit || 0,
    rating: row.rating || 3,
    status: row.status,
    remark: row.remark || ''
  }
  dialogVisible.value = true
}

const handleView = async (row) => {
  try {
    currentSupplier.value = await request.get(`/api/suppliers/${row.id}`)
    viewDialogVisible.value = true
  } catch (error) {
    ElMessage.error('获取供应商详情失败')
  }
}

const formatCreditLimit = (value) => {
  if (!value) return '0.00'
  // Decimal 可能被序列化为字符串或数字
  const num = typeof value === 'string' ? parseFloat(value) : value
  return num.toFixed(2)
}

const saveSupplier = async () => {
  if (!supplierForm.value.name) {
    ElMessage.warning('请输入供应商名称')
    return
  }

  try {
    const submitData = {
      ...supplierForm.value,
      credit_limit: supplierForm.value.credit_limit || null
    }

    if (isEdit.value) {
      await request.put(`/api/suppliers/${currentId.value}`, submitData)
      ElMessage.success('更新成功!')
    } else {
      await request.post('/api/suppliers', submitData)
      ElMessage.success('创建成功!')
    }
    dialogVisible.value = false
    await fetchSuppliers()
    await fetchStats()
  } catch (error) {
    const errorMsg = error.response?.data?.error || '操作失败'
    ElMessage.error(errorMsg)
  }
}

const deleteSupplier = async (id) => {
  try {
    await ElMessageBox.confirm('确定要删除这个供应商吗？', '警告', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    await request.delete(`/api/suppliers/${id}`)
    ElMessage.success('供应商已删除')
    await fetchSuppliers()
    await fetchStats()
  } catch (error) {
    if (error !== 'cancel') {
      const errorMsg = error.response?.data?.error || '删除失败'
      ElMessage.error(errorMsg)
    }
  }
}

onMounted(() => {
  const token = localStorage.getItem('erp_token')
  if (!token) {
    router.push('/login')
  } else {
    fetchSuppliers()
    fetchStats()
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