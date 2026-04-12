<template>
  <div style="padding: 20px;">
    <!-- 快捷入口卡片 -->
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

    <!-- 产品列表 -->
    <el-card shadow="never" class="search-card">
      <div class="table-toolbar">
        <div style="display: flex; gap: 10px; flex: 1;">
          <el-input
            v-model="searchKeyword"
            placeholder="搜索产品名称"
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
          <el-button type="success" @click="showImportDialog">
            <el-icon><Upload /></el-icon>
            批量导入
          </el-button>
          <el-button type="warning" @click="downloadTemplate">
            <el-icon><Download /></el-icon>
            下载模板
          </el-button>
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
            新增产品
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
        <el-table-column prop="name" label="产品名称" min-width="150" />
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
                type="primary"
                @click="goToInventoryAdjust(scope.row)"
              >
                <el-icon><Box /></el-icon>
                库存管理
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
      :title="isEdit ? '编辑产品' : '添加新产品'"
      width="600px"
      :close-on-click-modal="false"
    >
      <el-form :model="newItem" :rules="rules" ref="itemFormRef" label-width="100px">
        <el-form-item label="产品名称" prop="name">
          <el-input v-model="newItem.name" placeholder="请输入产品名称" />
        </el-form-item>
        <el-form-item label="SKU编号" prop="sku">
          <el-input v-model="newItem.sku" placeholder="请输入SKU编号" />
        </el-form-item>
        <el-form-item v-if="!isEdit" label="初始库存" prop="stock">
          <el-input-number v-model="newItem.stock" :min="0" style="width: 100%;" />
          <div style="color: #909399; font-size: 12px; margin-top: 5px;">
            提示：创建后请通过"库存管理"功能调整库存
          </div>
        </el-form-item>
        <el-form-item v-else label="当前库存">
          <div style="display: flex; align-items: center; gap: 12px;">
            <el-tag size="large" :type="getStockTagType(newItem.stock, newItem.min_stock)" effect="dark">
              {{ newItem.stock }}
            </el-tag>
            <el-button
              type="primary"
              size="small"
              @click="goToInventoryFromDialog"
              :icon="Box"
            >
              调整库存
            </el-button>
          </div>
          <div style="color: #909399; font-size: 12px; margin-top: 8px; line-height: 1.6;">
            <el-icon style="vertical-align: middle;"><InfoFilled /></el-icon>
            编辑模式下不能直接修改库存，请点击右侧按钮进行专业的库存调整操作
          </div>
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
            placeholder="请输入产品描述"
          />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" @click="saveItem" :loading="submitLoading">确认提交</el-button>
      </template>
    </el-dialog>

    <!-- 导入对话框 -->
    <el-dialog v-model="importDialogVisible" title="批量导入产品" width="600px">
      <el-alert
        title="导入说明"
        type="info"
        :closable="false"
        style="margin-bottom: 20px;"
      >
        <template #default>
          <div style="line-height: 1.8;">
            <p>1. 支持 CSV 和 Excel (.xlsx) 格式</p>
            <p>2. 文件格式：产品名称, SKU, 初始库存, 单位, 最低库存, 描述</p>
            <p>3. SKU 不能重复，重复的将被跳过</p>
            <p>4. 建议先下载模板，按照模板格式填写数据</p>
          </div>
        </template>
      </el-alert>

      <el-upload
        ref="uploadRef"
        drag
        :auto-upload="false"
        :on-change="handleFileChange"
        :limit="1"
        accept=".csv,.xlsx,.xls"
        style="width: 100%;"
      >
        <el-icon class="el-icon--upload"><UploadFilled /></el-icon>
        <div class="el-upload__text">
          拖拽文件到此处或 <em>点击上传</em>
        </div>
        <template #tip>
          <div class="el-upload__tip">
            只能上传 CSV 或 Excel 文件
          </div>
        </template>
      </el-upload>

      <div v-if="importResult" style="margin-top: 20px;">
        <el-divider />
        <h4>导入结果</h4>
        <el-descriptions :column="2" border>
          <el-descriptions-item label="成功数量">
            <el-tag type="success">{{ importResult.success_count }}</el-tag>
          </el-descriptions-item>
          <el-descriptions-item label="失败数量">
            <el-tag type="danger">{{ importResult.failed_count }}</el-tag>
          </el-descriptions-item>
        </el-descriptions>

        <div v-if="importResult.errors.length > 0" style="margin-top: 15px;">
          <h5>错误详情：</h5>
          <el-scrollbar height="200px">
            <div v-for="(error, index) in importResult.errors" :key="index" style="color: #f56c6c; padding: 5px 0;">
              {{ index + 1 }}. {{ error }}
            </div>
          </el-scrollbar>
        </div>
      </div>

      <template #footer>
        <el-button @click="closeImportDialog">关闭</el-button>
        <el-button type="primary" @click="submitImport" :loading="importing" :disabled="!selectedFile">
          开始导入
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Search, Plus, Edit, Delete, Ticket, ShoppingCart, Sell, Box, Coin, InfoFilled, Upload, Download, UploadFilled } from '@element-plus/icons-vue'
import request from '../utils/request'
import { API_ENDPOINTS, PAGINATION, ADJUSTMENT_TYPES, TIME_CONSTANTS } from '../api/constants'

const router = useRouter()
const items = ref([])
const loading = ref(false)
const submitLoading = ref(false)
const dialogVisible = ref(false)
const isEdit = ref(false)
const itemFormRef = ref(null)
const selectedIds = ref([])

const searchKeyword = ref('')
const searchSku = ref('')

const pagination = reactive({
  page: 1,
  pageSize: PAGINATION.DEFAULT_PAGE_SIZE,
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

const rules = {
  name: [{ required: true, message: '请输入产品名称', trigger: 'blur' }],
  sku: [{ required: true, message: '请输入SKU编号', trigger: 'blur' }],
  stock: [{ required: true, message: '请输入库存数量', trigger: 'blur' }]
}

const goToPage = (path) => {
  router.push(path)
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

    const res = await request.get(API_ENDPOINTS.ITEMS, { params })
    if (res.code === 200) {
      items.value = res.data.items
      pagination.total = res.data.total
    }
  } catch (error) {
    console.error('加载产品列表失败:', error)
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
      `确定要删除选中的 ${selectedIds.value.length} 个产品吗？`,
      '警告',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )

    await request.delete(API_ENDPOINTS.ITEMS, { data: selectedIds.value })
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
      const updateData = {
        name: newItem.name,
        sku: newItem.sku,
        unit: newItem.unit,
        min_stock: newItem.min_stock,
        description: newItem.description
      }
      await request.put(`${API_ENDPOINTS.ITEMS}/${newItem.id}`, updateData)
      ElMessage.success('更新成功')
    } else {
      await request.post(API_ENDPOINTS.ITEMS, newItem)
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
    await ElMessageBox.confirm('确定要删除这个产品吗？', '警告', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })

    await request.delete(`${API_ENDPOINTS.ITEMS}/${id}`)
    ElMessage.success('删除成功')
    loadItems()
  } catch (error) {
    if (error !== 'cancel') {
      console.error('删除失败:', error)
    }
  }
}

const goToInventoryAdjust = (item) => {
  router.push(`${API_ENDPOINTS.INVENTORY_ITEMS.replace('/items', '')}?id=${item.id}`)
}

const goToInventoryFromDialog = () => {
  dialogVisible.value = false
  setTimeout(() => {
    router.push(`${API_ENDPOINTS.INVENTORY_ITEMS.replace('/items', '')}?id=${newItem.id}`)
  }, TIME_CONSTANTS.DIALOG_CLOSE_DELAY)
}

const getStockType = (item) => {
  if (item.stock === 0) return 'danger'
  const minStock = item.min_stock || 0
  if (item.stock <= minStock) return 'warning'
  return 'success'
}

const getStockTagType = (stock, minStock) => {
  if (stock === 0) return 'danger'
  if (minStock && stock <= minStock) return 'warning'
  return 'success'
}

const handleSizeChange = (val) => {
  pagination.pageSize = val
  pagination.page = 1
  loadItems()
}

const handlePageChange = (val) => {
  pagination.page = val
  loadItems()
}

const importDialogVisible = ref(false)
const importResult = ref(null)
const selectedFile = ref(null)
const importing = ref(false)
const uploadRef = ref(null)

const showImportDialog = () => {
  importDialogVisible.value = true
  importResult.value = null
  selectedFile.value = null
}

const closeImportDialog = () => {
  importDialogVisible.value = false
  importResult.value = null
  selectedFile.value = null
  if (uploadRef.value) {
    uploadRef.value.clearFiles()
  }
}

const handleFileChange = (file) => {
  selectedFile.value = file.raw
}

const downloadTemplate = async () => {
  try {
    const response = await fetch(API_ENDPOINTS.IMPORT_TEMPLATE, {
      headers: {
        'Authorization': `Bearer ${localStorage.getItem('erp_token')}`
      }
    })
    const text = await response.text()

    const blob = new Blob([text], { type: 'text/csv;charset=utf-8;' })
    const link = document.createElement('a')
    link.href = URL.createObjectURL(blob)
    link.download = '产品导入模板.csv'
    link.click()

    ElMessage.success('模板下载成功')
  } catch (e) {
    console.error('下载模板失败', e)
    ElMessage.error('下载模板失败')
  }
}

const submitImport = async () => {
  if (!selectedFile.value) {
    ElMessage.warning('请选择要导入的文件')
    return
  }

  importing.value = true
  importResult.value = null

  try {
    const formData = new FormData()
    formData.append('file', selectedFile.value)

    const response = await fetch(API_ENDPOINTS.IMPORT_PRODUCTS, {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${localStorage.getItem('erp_token')}`
      },
      body: formData
    })

    const result = await response.json()
    importResult.value = result

    if (result.success_count > 0) {
      ElMessage.success(`导入完成！成功 ${result.success_count} 条`)
      loadItems()
    }

    if (result.failed_count > 0) {
      ElMessage.warning(`有 ${result.failed_count} 条记录导入失败，请查看错误详情`)
    }
  } catch (e) {
    console.error('导入失败', e)
    ElMessage.error('导入失败：' + e.message)
  } finally {
    importing.value = false
  }
}

onMounted(() => {
  const token = localStorage.getItem('erp_token')
  if (!token) {
    router.push('/login')
  } else {
    loadItems()
  }
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
