<template>
  <div style="padding: 20px;">
    <!-- 统计卡片 -->
    <el-row :gutter="20" style="margin-bottom: 20px;">
      <el-col :span="6">
        <el-card shadow="hover">
          <div class="stat-card">
            <div class="stat-icon" style="background-color: #409eff;">
              <el-icon :size="30"><Box /></el-icon>
            </div>
            <div class="stat-content">
              <div class="stat-value">{{ stats.total_items }}</div>
              <div class="stat-label">物料总数</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover">
          <div class="stat-card">
            <div class="stat-icon" style="background-color: #e6a23c;">
              <el-icon :size="30"><Warning /></el-icon>
            </div>
            <div class="stat-content">
              <div class="stat-value">{{ stats.low_stock_items }}</div>
              <div class="stat-label">低库存预警</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover">
          <div class="stat-card">
            <div class="stat-icon" style="background-color: #f56c6c;">
              <el-icon :size="30"><CircleClose /></el-icon>
            </div>
            <div class="stat-content">
              <div class="stat-value">{{ stats.out_of_stock_items }}</div>
              <div class="stat-label">缺货物料</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover">
          <div class="stat-card">
            <div class="stat-icon" style="background-color: #67c23a;">
              <el-icon :size="30"><TrendCharts /></el-icon>
            </div>
            <div class="stat-content">
              <div class="stat-value">{{ stats.total_stock_value }}</div>
              <div class="stat-label">总库存量</div>
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
            <el-input v-model="searchForm.keyword" placeholder="请输入物料名称" clearable style="width: 200px;" />
          </el-form-item>
          <el-form-item label="SKU" style="margin-bottom: 0;">
            <el-input v-model="searchForm.sku" placeholder="请输入SKU" clearable style="width: 180px;" />
          </el-form-item>
          <el-form-item style="margin-bottom: 0;">
            <el-checkbox v-model="searchForm.lowStock">仅显示低库存</el-checkbox>
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
        </div>
      </div>
    </el-card>

    <!-- 库存表格 -->
    <el-table :data="inventoryList" border style="width: 100%" v-loading="loading">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="name" label="物料名称" min-width="150" />
      <el-table-column prop="sku" label="SKU" width="150" />
      <el-table-column prop="stock" label="当前库存" width="120">
        <template #default="scope">
          <span :style="{ color: getStockColor(scope.row), fontWeight: 'bold' }">
            {{ scope.row.stock }}
          </span>
        </template>
      </el-table-column>
      <el-table-column prop="unit" label="单位" width="80" />
      <el-table-column prop="min_stock" label="最低库存" width="100" />
      <el-table-column label="库存状态" width="120">
        <template #default="scope">
          <el-tag :type="getStockStatusType(scope.row)">
            {{ getStockStatus(scope.row) }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column label="操作" width="280" fixed="right">
        <template #default="scope">
          <div style="display: flex; gap: 8px;">
            <el-button size="small" type="primary" @click="openAdjustDialog(scope.row)">
              调整库存
            </el-button>
            <el-button size="small" @click="viewHistory(scope.row)">
              调整记录
            </el-button>
          </div>
        </template>
      </el-table-column>
    </el-table>

    <!-- 库存调整对话框 -->
    <el-dialog v-model="adjustDialogVisible" title="库存调整" width="500px">
      <el-form :model="adjustForm" label-width="100px">
        <el-form-item label="物料名称">
          <el-input v-model="currentItem.name" disabled />
        </el-form-item>
        <el-form-item label="当前库存">
          <el-input v-model="currentItem.stock" disabled />
        </el-form-item>
        <el-form-item label="调整类型" required>
          <el-select v-model="adjustForm.adjustment_type" placeholder="请选择调整类型" style="width: 100%;">
            <el-option label="入库" value="入库" />
            <el-option label="出库" value="出库" />
            <el-option label="盘点调整" value="盘点调整" />
            <el-option label="损耗" value="损耗" />
            <el-option label="退货" value="退货" />
          </el-select>
        </el-form-item>
        <el-form-item label="调整数量" required>
          <el-input-number
            v-model="adjustForm.quantity"
            :min="1"
            :precision="0"
            controls-position="right"
            style="width: 100%;"
          />
        </el-form-item>
        <el-form-item label="调整原因">
          <el-input
            v-model="adjustForm.reason"
            type="textarea"
            :rows="3"
            placeholder="请输入调整原因（可选）"
          />
        </el-form-item>
        <el-form-item label="调整后库存">
          <el-input :value="calculateAfterStock" disabled />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="adjustDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="submitAdjustment">确认调整</el-button>
      </template>
    </el-dialog>

    <!-- 调整历史对话框 -->
    <el-dialog v-model="historyDialogVisible" title="库存调整记录" width="800px">
      <div v-if="currentItem">
        <h4 style="margin-bottom: 15px;">
          物料：{{ currentItem.name }} (SKU: {{ currentItem.sku }})
        </h4>
        <el-table :data="adjustmentHistory" border style="width: 100%">
          <el-table-column prop="id" label="ID" width="80" />
          <el-table-column prop="adjustment_type" label="调整类型" width="120" />
          <el-table-column prop="quantity" label="调整数量" width="100" />
          <el-table-column prop="before_stock" label="调整前" width="100" />
          <el-table-column prop="after_stock" label="调整后" width="100" />
          <el-table-column prop="reason" label="原因" min-width="150" show-overflow-tooltip />
          <el-table-column prop="created_at" label="调整时间" width="180" />
        </el-table>
      </div>
      <template #footer>
        <el-button @click="historyDialogVisible = false">关闭</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import request from '../utils/request'
import { ElMessage } from 'element-plus'
import { Box, Warning, CircleClose, TrendCharts, Search, Refresh } from '@element-plus/icons-vue'

const router = useRouter()
const route = useRoute()
const inventoryList = ref([])
const loading = ref(false)
const adjustDialogVisible = ref(false)
const historyDialogVisible = ref(false)
const currentItem = ref(null)
const adjustmentHistory = ref([])

const stats = ref({
  total_items: 0,
  low_stock_items: 0,
  out_of_stock_items: 0,
  total_stock_value: 0
})

const searchForm = ref({
  keyword: '',
  sku: '',
  lowStock: false
})

const adjustForm = ref({
  adjustment_type: '',
  quantity: 1,
  reason: ''
})

const calculateAfterStock = computed(() => {
  if (!currentItem.value) return 0

  const current = currentItem.value.stock
  const qty = adjustForm.value.quantity

  if (['入库', '盘点调整', '退货'].includes(adjustForm.value.adjustment_type)) {
    return current + qty
  } else if (['出库', '损耗'].includes(adjustForm.value.adjustment_type)) {
    return current - qty
  }

  return current
})

const fetchInventory = async () => {
  loading.value = true
  try {
    const params = {}
    if (searchForm.value.keyword) {
      params.keyword = searchForm.value.keyword
    }
    if (searchForm.value.sku) {
      params.sku = searchForm.value.sku
    }
    if (searchForm.value.lowStock) {
      params.low_stock = true
    }

    inventoryList.value = await request.get('/api/inventory', { params })
  } catch (e) {
    console.error('获取库存数据失败', e)
    ElMessage.error('获取库存数据失败')
  } finally {
    loading.value = false
  }
}

const fetchStats = async () => {
  try {
    stats.value = await request.get('/api/inventory/stats')
  } catch (e) {
    console.error('获取统计数据失败', e)
  }
}

const handleSearch = () => {
  fetchInventory()
}

const handleReset = () => {
  searchForm.value = {
    keyword: '',
    sku: '',
    lowStock: false
  }
  fetchInventory()
}

const openAdjustDialog = (item) => {
  currentItem.value = item
  adjustForm.value = {
    adjustment_type: '',
    quantity: 1,
    reason: ''
  }
  adjustDialogVisible.value = true
}

const submitAdjustment = async () => {
  if (!adjustForm.value.adjustment_type) {
    ElMessage.warning('请选择调整类型')
    return
  }

  if (adjustForm.value.quantity <= 0) {
    ElMessage.warning('调整数量必须大于0')
    return
  }

  try {
    await request.post(`/api/inventory/${currentItem.value.id}/adjust`, adjustForm.value)
    ElMessage.success('库存调整成功')
    adjustDialogVisible.value = false
    await fetchInventory()
    await fetchStats()
  } catch (error) {
    const errorMsg = error.response?.data?.error || '调整失败'
    ElMessage.error(errorMsg)
  }
}

const viewHistory = async (item) => {
  try {
    currentItem.value = item
    adjustmentHistory.value = await request.get(`/api/inventory/${item.id}/history`)
    historyDialogVisible.value = true
  } catch (error) {
    ElMessage.error('获取调整记录失败')
  }
}

const getStockColor = (item) => {
  if (item.stock === 0) return '#f56c6c'
  const minStock = item.min_stock || 0
  if (item.stock <= minStock) return '#e6a23c'
  return '#67c23a'
}

const getStockStatus = (item) => {
  if (item.stock === 0) return '缺货'
  const minStock = item.min_stock || 0
  if (item.stock <= minStock) return '低库存'
  return '正常'
}

const getStockStatusType = (item) => {
  if (item.stock === 0) return 'danger'
  const minStock = item.min_stock || 0
  if (item.stock <= minStock) return 'warning'
  return 'success'
}

onMounted(() => {
  const token = localStorage.getItem('erp_token')
  if (!token) {
    router.push('/login')
  } else {
    fetchInventory()
    fetchStats()

    // 如果 URL 中有 id 参数，自动打开该物料的库存调整对话框
    const itemId = route.query.id
    if (itemId) {
      // 等待数据加载完成后，找到对应的物料并打开对话框
      setTimeout(() => {
        const targetItem = inventoryList.value.find(item => item.id === parseInt(itemId))
        if (targetItem) {
          openAdjustDialog(targetItem)
        }
      }, 500)
    }
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
