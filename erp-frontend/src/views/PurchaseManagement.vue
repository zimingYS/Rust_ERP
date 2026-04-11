<template>
  <div style="padding: 20px;">
    <!-- 统计卡片 -->
    <el-row :gutter="20" style="margin-bottom: 20px;">
      <el-col :span="6">
        <el-card shadow="hover">
          <div class="stat-card">
            <div class="stat-icon" style="background-color: #409eff;">
              <el-icon :size="30"><Document /></el-icon>
            </div>
            <div class="stat-content">
              <div class="stat-value">{{ stats.total_orders }}</div>
              <div class="stat-label">总订单数</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover">
          <div class="stat-card">
            <div class="stat-icon" style="background-color: #e6a23c;">
              <el-icon :size="30"><Clock /></el-icon>
            </div>
            <div class="stat-content">
              <div class="stat-value">{{ stats.pending_approval }}</div>
              <div class="stat-label">待审批</div>
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
              <div class="stat-value">{{ stats.approved }}</div>
              <div class="stat-label">已审批</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover">
          <div class="stat-card">
            <div class="stat-icon" style="background-color: #f56c6c;">
              <el-icon :size="30"><Money /></el-icon>
            </div>
            <div class="stat-content">
              <div class="stat-value">¥{{ stats.this_month_amount.toFixed(2) }}</div>
              <div class="stat-label">本月金额</div>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <!-- 搜索和筛选 -->
    <el-card style="margin-bottom: 20px;">
      <el-form :inline="true" :model="searchForm">
        <el-form-item label="状态">
          <el-select v-model="searchForm.status" placeholder="请选择状态" clearable style="width: 150px;">
            <el-option label="待审批" value="待审批" />
            <el-option label="已审批" value="已审批" />
            <el-option label="已取消" value="已取消" />
          </el-select>
        </el-form-item>
        <el-form-item label="供应商">
          <el-input v-model="searchForm.supplier" placeholder="请输入供应商" clearable />
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
          <el-button type="primary" @click="openAddDialog">新建采购订单</el-button>
        </el-form-item>
      </el-form>
    </el-card>

    <!-- 订单表格 -->
    <el-table :data="orders" border style="width: 100%" v-loading="loading">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="order_number" label="订单编号" width="180" />
      <el-table-column prop="supplier" label="供应商" min-width="150" />
      <el-table-column prop="order_date" label="下单日期" width="180" />
      <el-table-column prop="expected_delivery_date" label="预计到货" width="180" />
      <el-table-column prop="total_amount" label="总金额" width="120">
        <template #default="scope">
          ¥{{ scope.row.total_amount.toFixed(2) }}
        </template>
      </el-table-column>
      <el-table-column prop="status" label="状态" width="100">
        <template #default="scope">
          <el-tag :type="getStatusType(scope.row.status)">
            {{ scope.row.status }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column label="操作" width="320" fixed="right">
        <template #default="scope">
          <div style="display: flex; gap: 8px;">
            <el-button size="small" @click="handleView(scope.row)">查看</el-button>
            <el-button
              size="small"
              @click="handleEdit(scope.row)"
              :disabled="scope.row.status === '已审批'"
            >
              编辑
            </el-button>
            <el-button
                v-if="scope.row.status === '待审批'"
                type="success"
                size="small"
                @click="handleApprove(scope.row.id)"
            >
              审批
            </el-button>
            <el-button
                v-if="scope.row.status === '待审批'"
                type="warning"
                size="small"
                @click="handleCancel(scope.row.id)"
            >
              取消
            </el-button>
            <el-button
              type="danger"
              size="small"
              @click="deleteOrder(scope.row.id)"
              :disabled="scope.row.status === '已审批'"
            >
              删除
            </el-button>
          </div>
        </template>
      </el-table-column>
    </el-table>

    <!-- 新建/编辑对话框 -->
    <el-dialog v-model="dialogVisible" :title="isEdit ? '编辑采购订单' : '新建采购订单'" width="70%">
      <el-form :model="newOrder" label-width="120px">
        <el-form-item label="供应商" required>
          <el-input v-model="newOrder.supplier" placeholder="请输入供应商名称" />
        </el-form-item>
        <el-form-item label="预计到货日期" required>
          <el-date-picker
              v-model="newOrder.expected_delivery_date"
              type="date"
              placeholder="选择日期"
              style="width: 100%;"
              value-format="YYYY-MM-DD"
          />
        </el-form-item>
        <el-form-item label="采购明细" required>
          <div class="items-container">
            <el-table :data="newOrder.items" border style="width: 100%">
              <el-table-column label="序号" width="60" align="center">
                <template #default="scope">
                  {{ scope.$index + 1 }}
                </template>
              </el-table-column>
              <el-table-column label="物品名称" min-width="180">
                <template #default="scope">
                  <el-input
                    v-model="scope.row.item_name"
                    placeholder="请输入物品名称"
                    clearable
                  />
                </template>
              </el-table-column>
              <el-table-column label="数量" width="120" align="center">
                <template #default="scope">
                  <el-input-number
                    v-model="scope.row.quantity"
                    :min="1"
                    :precision="0"
                    controls-position="right"
                    style="width: 100%;"
                  />
                </template>
              </el-table-column>
              <el-table-column label="单价(¥)" width="130" align="center">
                <template #default="scope">
                  <el-input-number
                    v-model="scope.row.unit_price"
                    :min="0"
                    :precision="2"
                    :step="0.01"
                    controls-position="right"
                    style="width: 100%;"
                  />
                </template>
              </el-table-column>
              <el-table-column label="小计(¥)" width="120" align="center">
                <template #default="scope">
                  <span class="subtotal">¥{{ (scope.row.quantity * scope.row.unit_price).toFixed(2) }}</span>
                </template>
              </el-table-column>
              <el-table-column label="操作" width="80" align="center">
                <template #default="scope">
                  <el-button
                    type="danger"
                    size="small"
                    link
                    @click="removeItem(scope.$index)"
                  >
                    删除
                  </el-button>
                </template>
              </el-table-column>
            </el-table>

            <div class="items-footer">
              <el-button type="primary" plain @click="addItem" :icon="Plus">
                添加明细
              </el-button>
              <div class="total-amount">
                订单总金额：<span class="amount">¥{{ calculateTotalAmount.toFixed(2) }}</span>
              </div>
            </div>
          </div>
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" @click="saveOrder">确认提交</el-button>
      </template>
    </el-dialog>

    <!-- 详情对话框 -->
    <el-dialog v-model="viewDialogVisible" title="采购订单详情" width="60%">
      <div v-if="currentOrder">
        <el-descriptions :column="2" border>
          <el-descriptions-item label="订单编号">{{ currentOrder.order_number }}</el-descriptions-item>
          <el-descriptions-item label="供应商">{{ currentOrder.supplier }}</el-descriptions-item>
          <el-descriptions-item label="下单日期">{{ currentOrder.order_date }}</el-descriptions-item>
          <el-descriptions-item label="预计到货">{{ currentOrder.expected_delivery_date }}</el-descriptions-item>
          <el-descriptions-item label="状态">
            <el-tag :type="getStatusType(currentOrder.status)">
              {{ currentOrder.status }}
            </el-tag>
          </el-descriptions-item>
          <el-descriptions-item label="总金额">¥{{ currentOrder.total_amount?.toFixed(2) }}</el-descriptions-item>
        </el-descriptions>

        <h3 style="margin-top: 20px;">采购明细</h3>
        <el-table :data="currentOrderItems" border style="width: 100%; margin-top: 10px;">
          <el-table-column prop="item_name" label="物品名称" />
          <el-table-column prop="quantity" label="数量" width="100" />
          <el-table-column prop="unit_price" label="单价" width="120">
            <template #default="scope">
              ¥{{ scope.row.unit_price.toFixed(2) }}
            </template>
          </el-table-column>
          <el-table-column prop="total_price" label="小计" width="120">
            <template #default="scope">
              ¥{{ scope.row.total_price.toFixed(2) }}
            </template>
          </el-table-column>
          <el-table-column prop="remark" label="备注" />
        </el-table>
      </div>
      <template #footer>
        <el-button @click="viewDialogVisible = false">关闭</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import request from '../utils/request'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Plus, Document, Clock, CircleCheck, Money } from '@element-plus/icons-vue'

const router = useRouter()
const orders = ref([])
const loading = ref(false)
const dialogVisible = ref(false)
const viewDialogVisible = ref(false)
const isEdit = ref(false)
const currentId = ref(null)
const currentOrder = ref(null)
const currentOrderItems = ref([])

const stats = ref({
  total_orders: 0,
  pending_approval: 0,
  approved: 0,
  total_amount: 0,
  this_month_amount: 0
})

const searchForm = ref({
  status: '',
  supplier: '',
  dateRange: []
})

const newOrder = ref({
  supplier: '',
  expected_delivery_date: '',
  items: []
})

const calculateTotalAmount = computed(() => {
  return newOrder.value.items.reduce((total, item) => {
    return total + (item.quantity * item.unit_price)
  }, 0)
})

const fetchOrders = async () => {
  loading.value = true
  try {
    const params = {}
    if (searchForm.value.status) {
      params.status = searchForm.value.status
    }
    if (searchForm.value.supplier) {
      params.supplier = searchForm.value.supplier
    }
    if (searchForm.value.dateRange && searchForm.value.dateRange.length === 2) {
      params.start_date = searchForm.value.dateRange[0]
      params.end_date = searchForm.value.dateRange[1]
    }

    orders.value = await request.get('/api/purchase-orders', { params })
  } catch (e) {
    console.error('获取数据失败', e)
    ElMessage.error('获取数据失败')
  } finally {
    loading.value = false
  }
}

const fetchStats = async () => {
  try {
    stats.value = await request.get('/api/purchase-orders/stats')
  } catch (e) {
    console.error('获取统计数据失败', e)
  }
}

const handleSearch = () => {
  fetchOrders()
}

const handleReset = () => {
  searchForm.value = {
    status: '',
    supplier: '',
    dateRange: []
  }
  fetchOrders()
}

const addItem = () => {
  newOrder.value.items.push({
    item_name: '',
    quantity: 1,
    unit_price: 0,
    remark: ''
  })
}

const removeItem = (index) => {
  newOrder.value.items.splice(index, 1)
}

const saveOrder = async () => {
  if (!newOrder.value.supplier) {
    ElMessage.warning('请输入供应商名称')
    return
  }
  if (!newOrder.value.expected_delivery_date) {
    ElMessage.warning('请选择预计到货日期')
    return
  }
  if (newOrder.value.items.length === 0) {
    ElMessage.warning('请至少添加一条采购明细')
    return
  }

  const hasEmptyItem = newOrder.value.items.some(item => !item.item_name)
  if (hasEmptyItem) {
    ElMessage.warning('请填写所有物品名称')
    return
  }

  try {
    if (isEdit.value) {
      await request.put(`/api/purchase-orders/${currentId.value}`, {
        supplier: newOrder.value.supplier,
        expected_delivery_date: newOrder.value.expected_delivery_date,
        status: currentOrder.value?.status || '待审批',
        items: newOrder.value.items
      })
      ElMessage.success('更新成功!')
    } else {
      await request.post('/api/purchase-orders', newOrder.value)
      ElMessage.success('创建成功!')
    }
    dialogVisible.value = false
    fetchOrders()
    fetchStats()
  } catch (error) {
    const errorMsg = error.response?.data?.error || '操作失败'
    ElMessage.error(errorMsg)
  }
}

const deleteOrder = async (id) => {
  try {
    await ElMessageBox.confirm('确定要删除这个采购订单吗？', '警告', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    await request.delete(`/api/purchase-orders/${id}`)
    ElMessage.success('订单已删除')
    fetchOrders()
    fetchStats()
  } catch (error) {
    if (error !== 'cancel') {
      const errorMsg = error.response?.data?.error || '删除失败'
      ElMessage.error(errorMsg)
    }
  }
}

const handleApprove = async (id) => {
  try {
    await ElMessageBox.confirm('确定要审批通过这个采购订单吗？', '提示', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'info'
    })
    await request.post(`/api/purchase-orders/${id}/approve`)
    ElMessage.success('审批成功!')
    fetchOrders()
    fetchStats()
  } catch (error) {
    if (error !== 'cancel') {
      const errorMsg = error.response?.data?.error || '审批失败'
      ElMessage.error(errorMsg)
    }
  }
}

const handleCancel = async (id) => {
  try {
    await ElMessageBox.confirm('确定要取消这个采购订单吗？', '提示', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    await request.post(`/api/purchase-orders/${id}/cancel`)
    ElMessage.success('订单已取消')
    fetchOrders()
    fetchStats()
  } catch (error) {
    if (error !== 'cancel') {
      const errorMsg = error.response?.data?.error || '取消失败'
      ElMessage.error(errorMsg)
    }
  }
}

const handleView = async (row) => {
  try {
    const [order, items] = await request.get(`/api/purchase-orders/${row.id}`)
    currentOrder.value = order
    currentOrderItems.value = items
    viewDialogVisible.value = true
  } catch (error) {
    ElMessage.error('获取订单详情失败')
  }
}

const handleEdit = async (row) => {
  try {
    const [order, items] = await request.get(`/api/purchase-orders/${row.id}`)
    isEdit.value = true
    currentId.value = row.id
    currentOrder.value = order
    newOrder.value = {
      supplier: order.supplier,
      expected_delivery_date: order.expected_delivery_date,
      items: items.map(item => ({
        item_name: item.item_name,
        quantity: item.quantity,
        unit_price: item.unit_price,
        remark: item.remark
      }))
    }
    dialogVisible.value = true
  } catch (error) {
    ElMessage.error('获取订单详情失败')
  }
}

const openAddDialog = () => {
  isEdit.value = false
  currentId.value = null
  currentOrder.value = null
  newOrder.value = {
    supplier: '',
    expected_delivery_date: '',
    items: []
  }
  dialogVisible.value = true
}

const getStatusType = (status) => {
  switch (status) {
    case '待审批':
      return 'warning'
    case '已审批':
      return 'success'
    case '已取消':
      return 'danger'
    default:
      return 'info'
  }
}

onMounted(() => {
  const token = localStorage.getItem('erp_token')
  if (!token) {
    router.push('/login')
  } else {
    fetchOrders()
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

.items-container {
  width: 100%;
}

.items-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 15px;
  padding: 10px 0;
}

.total-amount {
  font-size: 16px;
  color: #606266;
  font-weight: 500;
}

.amount {
  color: #f56c6c;
  font-size: 20px;
  font-weight: bold;
}

.subtotal {
  color: #409eff;
  font-weight: 500;
}

:deep(.el-input-number) {
  width: 100%;
}

:deep(.el-input-number .el-input__inner) {
  text-align: center;
}
</style>
