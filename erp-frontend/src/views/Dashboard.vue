<template>
  <div style="padding: 20px;">
    <el-row :gutter="20" style="margin-bottom: 20px;">
      <el-col :span="6">
        <el-card shadow="hover" class="quick-access-card" @click="goToPage('/purchase')">
          <div class="stat-card">
            <div class="stat-icon" style="background-color: #409eff;">
              <el-icon :size="30"><ShoppingCart /></el-icon>
            </div>
            <div class="stat-content">
              <div class="stat-label">采购管理</div>
              <div class="stat-desc">创建采购订单</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover" class="quick-access-card" @click="goToPage('/sales')">
          <div class="stat-card">
            <div class="stat-icon" style="background-color: #67c23a;">
              <el-icon :size="30"><Sell /></el-icon>
            </div>
            <div class="stat-content">
              <div class="stat-label">销售管理</div>
              <div class="stat-desc">创建销售订单</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover" class="quick-access-card" @click="goToPage('/inventory')">
          <div class="stat-card">
            <div class="stat-icon" style="background-color: #e6a23c;">
              <el-icon :size="30"><Box /></el-icon>
            </div>
            <div class="stat-content">
              <div class="stat-label">库存管理</div>
              <div class="stat-desc">查看库存状态</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover" class="quick-access-card" @click="goToPage('/finance')">
          <div class="stat-card">
            <div class="stat-icon" style="background-color: #f56c6c;">
              <el-icon :size="30"><Coin /></el-icon>
            </div>
            <div class="stat-content">
              <div class="stat-label">财务管理</div>
              <div class="stat-desc">查看财务记录</div>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <el-card shadow="never" class="search-card">
      <div class="table-toolbar">
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
        <el-table-column label="操作" width="350" fixed="right">
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
              <el-button
                size="small"
                type="warning"
                @click="goToInventory(scope.row.id)"
              >
                <el-icon><Box /></el-icon>
                查库存
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
import { ref, reactive, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Search, Plus, Edit, Delete, Refresh, Ticket, ShoppingCart, Sell, Box, Coin } from '@element-plus/icons-vue'
import request from '../utils/request'

const router = useRouter()
const items = ref([])
const loading = ref(false)
const submitLoading = ref(false)
const stockSubmitLoading = ref(false)
const dialogVisible = ref(false)
const stockDialogVisible = ref(false)
const isEdit = ref(false)
const itemFormRef = ref(null)
const selectedIds = ref([])

const searchKeyword = ref('')
const searchSku = ref('')

const pagination = reactive({
  page: 1,
  pageSize: 10,
  total: 0
})

const newItem = reactive({
  name: '',
  sku: '',
  stock: 0,
  unit: '',
  min_stock: 0,
  description: ''
})

const currentItem = reactive({
  id: 0,
  name: '',
  stock: 0
})

const stockAdjustment = reactive({
  adjustment: 0,
  reason: ''
})

const rules = {
  name: [{ required: true, message: '请输入物料名称', trigger: 'blur' }],
  sku: [{ required: true, message: '请输入SKU编号', trigger: 'blur' }],
  stock: [{ required: true, message: '请输入库存数量', trigger: 'blur' }]
}

const goToPage = (path) => {
  router.push(path)
}

const goToInventory = (itemId) => {
  router.push(`/inventory?id=${itemId}`)
}

const loadItems = async () => {
  loading.value = true
  try {
    const params = {
      page: pagination.page,
      page_size: pagination.pageSize
    }
    if (searchKeyword.value) {
      params.keyword = searchKeyword.value
    }
    if (searchSku.value) {
      params.sku = searchSku.value
    }

    const res = await request.get('/api/items', { params })
    if (res.code === 200) {
      items.value = res.data.items
      pagination.total = res.data.total
    }
  } catch (error) {
    console.error('加载物料列表失败:', error)
  } finally {
    loading.value = false
  }
}

const handleSearch = () => {
  pagination.page = 1
  loadItems()
}

const handleReset = () => {
  searchKeyword.value = ''
  searchSku.value = ''
  pagination.page = 1
  loadItems()
}

const handleSelectionChange = (selection) => {
  selectedIds.value = selection.map(item => item.id)
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

    await request.delete('/api/items', { data: selectedIds.value })
    ElMessage.success('批量删除成功')
    selectedIds.value = []
    loadItems()
  } catch (error) {
    if (error !== 'cancel') {
      console.error('批量删除失败:', error)
    }
  }
}

const openAddDialog = () => {
  isEdit.value = false
  Object.assign(newItem, {
    name: '',
    sku: '',
    stock: 0,
    unit: '',
    min_stock: 0,
    description: ''
  })
  dialogVisible.value = true
}

const handleEdit = (row) => {
  isEdit.value = true
  Object.assign(newItem, row)
  dialogVisible.value = true
}

const saveItem = async () => {
  if (!itemFormRef.value) return

  try {
    await itemFormRef.value.validate()
    submitLoading.value = true

    if (isEdit.value) {
      await request.put(`/api/items/${newItem.id}`, newItem)
      ElMessage.success('更新成功')
    } else {
      await request.post('/api/items', newItem)
      ElMessage.success('创建成功')
    }

    dialogVisible.value = false
    loadItems()
  } catch (error) {
    if (error !== false) {
      console.error('保存失败:', error)
    }
  } finally {
    submitLoading.value = false
  }
}

const deleteItem = async (id) => {
  try {
    await ElMessageBox.confirm('确定要删除该物料吗？', '警告', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })

    await request.delete(`/api/items/${id}`)
    ElMessage.success('删除成功')
    loadItems()
  } catch (error) {
    if (error !== 'cancel') {
      console.error('删除失败:', error)
    }
  }
}

const openStockDialog = (row) => {
  Object.assign(currentItem, row)
  stockAdjustment.adjustment = 0
  stockAdjustment.reason = ''
  stockDialogVisible.value = true
}

const getAdjustedStockType = () => {
  const adjustedStock = currentItem.stock + stockAdjustment.adjustment
  if (adjustedStock < 0) return 'danger'
  if (adjustedStock === 0) return 'info'
  if (adjustedStock <= currentItem.min_stock) return 'warning'
  return 'success'
}

const saveStockAdjustment = async () => {
  if (stockAdjustment.adjustment === 0) {
    ElMessage.warning('调整数量不能为0')
    return
  }

  try {
    stockSubmitLoading.value = true
    await request.post(`/api/items/${currentItem.id}/stock`, stockAdjustment)
    ElMessage.success('库存调整成功')
    stockDialogVisible.value = false
    loadItems()
  } catch (error) {
    console.error('库存调整失败:', error)
  } finally {
    stockSubmitLoading.value = false
  }
}

const getStockType = (row) => {
  if (row.stock === 0) return 'danger'
  if (row.min_stock && row.stock <= row.min_stock) return 'warning'
  return 'success'
}

const handleSizeChange = (val) => {
  pagination.pageSize = val
  loadItems()
}

const handlePageChange = (val) => {
  pagination.page = val
  loadItems()
}

onMounted(() => {
  loadItems()
})
</script>

<style scoped>
.quick-access-card {
  cursor: pointer;
  transition: all 0.3s;
}

.quick-access-card:hover {
  transform: translateY(-5px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

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

.stat-label {
  font-size: 16px;
  font-weight: bold;
  color: #303133;
  margin-bottom: 5px;
}

.stat-desc {
  font-size: 12px;
  color: #909399;
}

.search-card {
  margin-bottom: 20px;
}

.table-toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
  flex-wrap: wrap;
  gap: 10px;
}
</style>
