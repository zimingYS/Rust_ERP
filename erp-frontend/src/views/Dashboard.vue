<template>
  <div style="padding: 20px;">
    <el-card shadow="never">
      <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;">
        <div style="display: flex; gap: 10px; flex: 1;">
          <el-input
            v-model="searchKeyword"
            placeholder="搜索物料名称"
            clearable
            style="width: 250px;"
            @clear="handleSearch"
          >
            <template #prefix>
              <el-icon><Search /></el-icon>
            </template>
          </el-input>
          <el-input
            v-model="searchSku"
            placeholder="搜索SKU"
            clearable
            style="width: 200px;"
            @clear="handleSearch"
          >
            <template #prefix>
              <el-icon><Ticket /></el-icon>
            </template>
          </el-input>
          <el-button type="primary" @click="handleSearch">
            <el-icon><Search /></el-icon>
            搜索
          </el-button>
          <el-button @click="handleReset">重置</el-button>
        </div>
        <div style="display: flex; gap: 10px;">
          <el-button
            type="danger"
            :disabled="selectedIds.length === 0"
            @click="handleBatchDelete"
          >
            <el-icon><Delete /></el-icon>
            批量删除 ({{ selectedIds.length }})
          </el-button>
          <el-button type="primary" @click="openAddDialog">
            <el-icon><Plus /></el-icon>
            新增物料
          </el-button>
        </div>
      </div>

      <el-table
        :data="items"
        border
        style="width: 100%"
        @selection-change="handleSelectionChange"
        v-loading="loading"
      >
        <el-table-column type="selection" width="55" />
        <el-table-column prop="id" label="ID" width="80" />
        <el-table-column prop="name" label="物料名称" min-width="150" />
        <el-table-column prop="sku" label="SKU编号" width="150" />
        <el-table-column prop="stock" label="当前库存" width="120" align="center">
          <template #default="scope">
            <el-tag :type="getStockType(scope.row)">
              {{ scope.row.stock }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="unit" label="单位" width="80" align="center">
          <template #default="scope">
            {{ scope.row.unit || '-' }}
          </template>
        </el-table-column>
        <el-table-column prop="min_stock" label="最低库存" width="100" align="center">
          <template #default="scope">
            {{ scope.row.min_stock || '-' }}
          </template>
        </el-table-column>
        <el-table-column prop="description" label="描述" min-width="200" show-overflow-tooltip>
          <template #default="scope">
            {{ scope.row.description || '-' }}
          </template>
        </el-table-column>
        <el-table-column label="操作" width="280" fixed="right">
          <template #default="scope">
            <div style="display: flex; gap: 8px;">
              <el-button size="small" @click="handleEdit(scope.row)">
                <el-icon><Edit /></el-icon>
                编辑
              </el-button>
              <el-button
                size="small"
                type="success"
                @click="openStockDialog(scope.row)"
              >
                <el-icon><Refresh /></el-icon>
                调库存
              </el-button>
              <el-button type="danger" size="small" @click="deleteItem(scope.row.id)">
                <el-icon><Delete /></el-icon>
                删除
              </el-button>
            </div>
          </template>
        </el-table-column>
      </el-table>

      <div style="margin-top: 20px; display: flex; justify-content: flex-end;">
        <el-pagination
          v-model:current-page="pagination.page"
          v-model:page-size="pagination.pageSize"
          :page-sizes="[10, 20, 50, 100]"
          :total="pagination.total"
          layout="total, sizes, prev, pager, next, jumper"
          @size-change="handleSizeChange"
          @current-change="handlePageChange"
        />
      </div>
    </el-card>

    <el-dialog
      v-model="dialogVisible"
      :title="isEdit ? '编辑物料' : '添加新物料'"
      width="600px"
      :close-on-click-modal="false"
    >
      <el-form :model="newItem" :rules="rules" ref="itemFormRef" label-width="100px">
        <el-form-item label="物料名称" prop="name">
          <el-input v-model="newItem.name" placeholder="请输入物料名称" />
        </el-form-item>
        <el-form-item label="SKU编号" prop="sku">
          <el-input v-model="newItem.sku" placeholder="请输入SKU编号" />
        </el-form-item>
        <el-form-item label="当前库存" prop="stock">
          <el-input-number v-model="newItem.stock" :min="0" style="width: 100%;" />
        </el-form-item>
        <el-form-item label="单位" prop="unit">
          <el-input v-model="newItem.unit" placeholder="如：个、件、kg" />
        </el-form-item>
        <el-form-item label="最低库存" prop="min_stock">
          <el-input-number v-model="newItem.min_stock" :min="0" style="width: 100%;" />
        </el-form-item>
        <el-form-item label="描述" prop="description">
          <el-input
            v-model="newItem.description"
            type="textarea"
            :rows="3"
            placeholder="请输入物料描述"
          />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" @click="saveItem" :loading="submitLoading">确认提交</el-button>
      </template>
    </el-dialog>

    <el-dialog
      v-model="stockDialogVisible"
      title="库存调整"
      width="500px"
      :close-on-click-modal="false"
    >
      <el-form :model="stockAdjustment" label-width="100px">
        <el-form-item label="物料名称">
          <el-input v-model="currentItem.name" disabled />
        </el-form-item>
        <el-form-item label="当前库存">
          <el-tag size="large">{{ currentItem.stock }}</el-tag>
        </el-form-item>
        <el-form-item label="调整数量" required>
          <el-input-number
            v-model="stockAdjustment.adjustment"
            style="width: 100%;"
            placeholder="正数增加，负数减少"
          />
          <div style="color: #909399; font-size: 12px; margin-top: 5px;">
            提示：输入正数增加库存，负数减少库存
          </div>
        </el-form-item>
        <el-form-item label="调整原因">
          <el-input
            v-model="stockAdjustment.reason"
            type="textarea"
            :rows="3"
            placeholder="请输入调整原因（可选）"
          />
        </el-form-item>
        <el-form-item label="调整后库存">
          <el-tag size="large" :type="getAdjustedStockType()">
            {{ currentItem.stock + stockAdjustment.adjustment }}
          </el-tag>
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="stockDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="saveStockAdjustment" :loading="stockSubmitLoading">
          确认调整
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import request from '../utils/request'
import { ElMessage, ElMessageBox } from 'element-plus'
import {
  Search,
  Ticket,
  Plus,
  Delete,
  Edit,
  Refresh
} from '@element-plus/icons-vue'

const router = useRouter()
const items = ref([])
const loading = ref(false)
const submitLoading = ref(false)
const stockSubmitLoading = ref(false)
const dialogVisible = ref(false)
const stockDialogVisible = ref(false)
const isEdit = ref(false)
const currentId = ref(null)
const selectedIds = ref([])
const itemFormRef = ref(null)

const searchKeyword = ref('')
const searchSku = ref('')

const pagination = ref({
  page: 1,
  pageSize: 10,
  total: 0
})

const newItem = ref({
  name: '',
  sku: '',
  stock: 0,
  unit: '',
  min_stock: 0,
  description: ''
})

const currentItem = ref({})
const stockAdjustment = ref({
  adjustment: 0,
  reason: ''
})

const rules = {
  name: [
    { required: true, message: '请输入物料名称', trigger: 'blur' },
    { min: 2, max: 100, message: '长度在 2 到 100 个字符', trigger: 'blur' }
  ],
  sku: [
    { required: true, message: '请输入SKU编号', trigger: 'blur' },
    { min: 2, max: 50, message: '长度在 2 到 50 个字符', trigger: 'blur' }
  ],
  stock: [
    { required: true, message: '请输入库存数量', trigger: 'blur' }
  ]
}

const fetchItems = async () => {
  loading.value = true
  try {
    const params = {
      page: pagination.value.page,
      page_size: pagination.value.pageSize
    }

    if (searchKeyword.value) {
      params.keyword = searchKeyword.value
    }
    if (searchSku.value) {
      params.sku = searchSku.value
    }

    const response = await request.get('/api/items', { params })
    items.value = response.data.items
    pagination.value.total = response.data.total
  } catch (e) {
    console.error('获取数据失败', e)
    ElMessage.error('获取数据失败')
  } finally {
    loading.value = false
  }
}

const handleSearch = () => {
  pagination.value.page = 1
  fetchItems()
}

const handleReset = () => {
  searchKeyword.value = ''
  searchSku.value = ''
  pagination.value.page = 1
  fetchItems()
}

const handlePageChange = (page) => {
  pagination.value.page = page
  fetchItems()
}

const handleSizeChange = (pageSize) => {
  pagination.value.pageSize = pageSize
  pagination.value.page = 1
  fetchItems()
}

const handleSelectionChange = (selection) => {
  selectedIds.value = selection.map(item => item.id)
}

const saveItem = async () => {
  if (!itemFormRef.value) return

  await itemFormRef.value.validate(async (valid) => {
    if (!valid) return

    submitLoading.value = true
    try {
      if (isEdit.value) {
        await request.put(`/api/items/${currentId.value}`, newItem.value)
        ElMessage.success('更新成功!')
      } else {
        await request.post('/api/items', newItem.value)
        ElMessage.success('添加成功!')
      }
      dialogVisible.value = false
      fetchItems()
    } catch (error) {
      const errorMsg = error.response?.data?.message || '操作失败'
      ElMessage.error(errorMsg)
    } finally {
      submitLoading.value = false
    }
  })
}

const deleteItem = async (id) => {
  try {
    await ElMessageBox.confirm('确定要删除这个物料吗？', '警告', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    await request.delete(`/api/items/${id}`)
    ElMessage.success('物料已成功删除')
    fetchItems()
  } catch (error) {
    if (error !== 'cancel') {
      const errorMsg = error.response?.data?.message || '删除失败'
      ElMessage.error(errorMsg)
    }
  }
}

const handleBatchDelete = async () => {
  try {
    await ElMessageBox.confirm(
      `确定要删除选中的 ${selectedIds.value.length} 个物料吗？`,
      '警告',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )
    const response = await request.delete('/api/items', { data: selectedIds.value })
    ElMessage.success(response.message || '批量删除成功')
    selectedIds.value = []
    fetchItems()
  } catch (error) {
    if (error !== 'cancel') {
      const errorMsg = error.response?.data?.message || '批量删除失败'
      ElMessage.error(errorMsg)
    }
  }
}

const handleEdit = (row) => {
  isEdit.value = true
  currentId.value = row.id
  newItem.value = {
    name: row.name,
    sku: row.sku,
    stock: row.stock,
    unit: row.unit || '',
    min_stock: row.min_stock || 0,
    description: row.description || ''
  }
  dialogVisible.value = true
}

const openAddDialog = () => {
  isEdit.value = false
  currentId.value = null
  newItem.value = {
    name: '',
    sku: '',
    stock: 0,
    unit: '',
    min_stock: 0,
    description: ''
  }
  dialogVisible.value = true
}

const openStockDialog = (row) => {
  currentItem.value = row
  stockAdjustment.value = {
    adjustment: 0,
    reason: ''
  }
  stockDialogVisible.value = true
}

const saveStockAdjustment = async () => {
  if (stockAdjustment.value.adjustment === 0) {
    ElMessage.warning('请输入调整数量')
    return
  }

  stockSubmitLoading.value = true
  try {
    await request.post(`/api/items/${currentItem.value.id}/stock`, stockAdjustment.value)
    ElMessage.success('库存调整成功!')
    stockDialogVisible.value = false
    fetchItems()
  } catch (error) {
    const errorMsg = error.response?.data?.message || '库存调整失败'
    ElMessage.error(errorMsg)
  } finally {
    stockSubmitLoading.value = false
  }
}

const getStockType = (row) => {
  if (row.min_stock && row.stock <= row.min_stock) {
    return 'danger'
  }
  if (row.stock === 0) {
    return 'info'
  }
  return 'success'
}

const getAdjustedStockType = () => {
  const adjustedStock = currentItem.value.stock + stockAdjustment.value.adjustment
  if (adjustedStock < 0) {
    return 'danger'
  }
  if (currentItem.value.min_stock && adjustedStock <= currentItem.value.min_stock) {
    return 'warning'
  }
  return 'success'
}

onMounted(() => {
  const token = localStorage.getItem('erp_token')
  if (!token) {
    router.push('/login')
  } else {
    fetchItems()
  }
})
</script>

<style scoped>
.el-tag {
  font-weight: 500;
}
</style>
