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
              <div class="stat-value">{{ stats.pending_confirm }}</div>
              <div class="stat-label">待确认</div>
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
              <div class="stat-value">{{ stats.confirmed }}</div>
              <div class="stat-label">已确认</div>
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
              <div class="stat-value">¥{{ (stats.this_month_amount || 0).toFixed(2) }}</div>
              <div class="stat-label">本月金额</div>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <!-- 搜索和筛选 -->
    <el-card style="margin-bottom: 20px;">
      <div style="display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 10px;">
        <el-form :inline="true" :model="searchForm" style="margin: 0;">
          <el-form-item label="状态" style="margin-bottom: 0;">
            <el-select v-model="searchForm.status" placeholder="请选择状态" clearable style="width: 150px;">
              <el-option label="待确认" value="待确认" />
              <el-option label="已确认" value="已确认" />
              <el-option label="已完成" value="已完成" />
              <el-option label="已取消" value="已取消" />
            </el-select>
          </el-form-item>
          <el-form-item label="客户" style="margin-bottom: 0;">
            <el-input v-model="searchForm.customer" placeholder="请输入客户" clearable style="width: 200px;" />
          </el-form-item>
          <el-form-item label="日期范围" style="margin-bottom: 0;">
            <el-date-picker
              v-model="searchForm.dateRange"
              type="daterange"
              range-separator="至"
              start-placeholder="开始日期"
              end-placeholder="结束日期"
              value-format="YYYY-MM-DD"
              style="width: 240px;"
            />
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
            新建销售订单
          </el-button>
        </div>
      </div>
    </el-card>

    <!-- 订单表格 -->
    <el-table :data="orders" border style="width: 100%" v-loading="loading">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="order_number" label="订单编号" width="180" />
      <el-table-column prop="customer" label="客户" min-width="150" />
      <el-table-column prop="order_date" label="下单日期" width="180" />
      <el-table-column prop="delivery_date" label="预计交付" width="180" />
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
      <el-table-column label="操作" width="320">
        <template #default="scope">
          <div style="display: flex; gap: 6px; flex-wrap: wrap;">
            <el-button size="small" @click="handleView(scope.row)">查看</el-button>
            <el-button
              size="small"
              @click="handleEdit(scope.row)"
              :disabled="scope.row.status === '已确认' || scope.row.status === '已完成'"
            >
              编辑
            </el-button>
            <el-button
                v-if="scope.row.status === '待确认' && hasApprovePermission"
                type="success"
                size="small"
                @click="handleConfirm(scope.row.id)"
            >
              确认
            </el-button>
            <el-button
                v-if="scope.row.status === '已确认' && hasApprovePermission"
                type="primary"
                size="small"
                @click="handleComplete(scope.row.id)"
            >
              完成
            </el-button>
            <el-button
                v-if="scope.row.status === '待确认' && hasApprovePermission"
                type="warning"
                size="small"
                @click="handleCancel(scope.row.id)"
            >
              取消
            </el-button>
            <el-button
              type="danger"
              size="small"
              @click="deleteOrder(scope.row.id, scope.row.status)"
              :disabled="(scope.row.status === '已确认' || scope.row.status === '已完成') && !isAdmin"
            >
              {{ (scope.row.status === '已确认' || scope.row.status === '已完成') ? '强制删除' : '删除' }}
            </el-button>
          </div>
        </template>
      </el-table-column>
    </el-table>

    <!-- 新建/编辑对话框 -->
    <el-dialog v-model="dialogVisible" :title="isEdit ? '编辑销售订单' : '新建销售订单'" width="900px" :close-on-click-modal="false">
      <el-form :model="newOrder" label-width="120px" class="sales-form">
        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item label="客户" required>
              <el-select
                v-model="newOrder.customer_id"
                placeholder="请选择客户"
                filterable
                @change="handleCustomerChange"
                style="width: 100%;"
              >
                <el-option
                  v-for="customer in customerList"
                  :key="customer.id"
                  :label="customer.name"
                  :value="customer.id"
                >
                  <div style="display: flex; justify-content: space-between; align-items: center;">
                    <span>{{ customer.name }}</span>
                    <span style="color: #909399; font-size: 12px;">
                      {{ customer.contact_person }} {{ customer.phone ? `| ${customer.phone}` : '' }}
                    </span>
                  </div>
                </el-option>
              </el-select>
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="预计交付日期" required>
              <el-date-picker
                v-model="newOrder.delivery_date"
                type="date"
                placeholder="选择日期"
                style="width: 100%;"
                value-format="YYYY-MM-DD"
              />
            </el-form-item>
          </el-col>
        </el-row>

        <el-divider content-position="left">
          <el-icon><Sell /></el-icon>
          <span style="margin-left: 8px;">销售明细</span>
        </el-divider>

        <el-form-item label="销售明细" required>
          <div class="items-container">
            <el-table :data="newOrder.items" border style="width: 100%" max-height="400">
              <el-table-column label="序号" width="60" align="center">
                <template #default="scope">
                  <span class="item-index">{{ scope.$index + 1 }}</span>
                </template>
              </el-table-column>
              <el-table-column label="物品名称" min-width="220">
                <template #default="scope">
                  <el-select
                    v-model="scope.row.item_id"
                    placeholder="请选择物料"
                    filterable
                    clearable
                    @change="(val) => handleItemChange(scope.$index, val)"
                    style="width: 100%;"
                  >
                    <el-option
                      v-for="item in allItems"
                      :key="item.id"
                      :label="`${item.name} (${item.sku})`"
                      :value="item.id"
                    />
                  </el-select>
                </template>
              </el-table-column>
              <el-table-column label="数量" width="140" align="center">
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
              <el-table-column label="单价(¥)" width="150" align="center">
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
              <el-table-column label="小计(¥)" width="140" align="center">
                <template #default="scope">
                  <span class="subtotal">¥{{ (scope.row.quantity * scope.row.unit_price).toFixed(2) }}</span>
                </template>
              </el-table-column>
              <el-table-column label="操作" width="100" align="center" fixed="right">
                <template #default="scope">
                  <el-button
                    type="danger"
                    size="small"
                    link
                    @click="removeItem(scope.$index)"
                  >
                    <el-icon><Delete /></el-icon>
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
                <span class="label">订单总金额：</span>
                <span class="amount">¥{{ calculateTotalAmount.toFixed(2) }}</span>
              </div>
            </div>
          </div>
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" @click="saveOrder" :loading="submitLoading">确认提交</el-button>
      </template>
    </el-dialog>

    <!-- 详情对话框 -->
    <el-dialog v-model="viewDialogVisible" title="销售订单详情" width="60%">
      <div v-if="currentOrder">
        <el-descriptions :column="2" border>
          <el-descriptions-item label="订单编号">{{ currentOrder.order_number }}</el-descriptions-item>
          <el-descriptions-item label="客户">{{ currentOrder.customer }}</el-descriptions-item>
          <el-descriptions-item label="下单日期">{{ currentOrder.order_date }}</el-descriptions-item>
          <el-descriptions-item label="预计交付">{{ currentOrder.delivery_date }}</el-descriptions-item>
          <el-descriptions-item label="状态">
            <el-tag :type="getStatusType(currentOrder.status)">
              {{ currentOrder.status }}
            </el-tag>
          </el-descriptions-item>
          <el-descriptions-item label="总金额">¥{{ currentOrder.total_amount?.toFixed(2) }}</el-descriptions-item>
        </el-descriptions>

        <h3 style="margin-top: 20px;">销售明细</h3>
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
import { Plus, Document, Clock, CircleCheck, Money, Search, Refresh, Sell, Delete } from '@element-plus/icons-vue'

const router = useRouter()
const orders = ref([])
const loading = ref(false)
const submitLoading = ref(false)
const dialogVisible = ref(false)
const viewDialogVisible = ref(false)
const isEdit = ref(false)
const currentId = ref(null)
const currentOrder = ref(null)
const currentOrderItems = ref([])
const allItems = ref([])
const customerList = ref([])

const userRole = ref(parseInt(localStorage.getItem('user_role') || '3'))
const hasApprovePermission = computed(() => userRole.value === 1 || userRole.value === 2)
const isAdmin = computed(() => userRole.value === 1)

const stats = ref({
  total_orders: 0,
  pending_confirm: 0,
  confirmed: 0,
  total_amount: 0,
  this_month_amount: 0
})

const searchForm = ref({
  status: '',
  customer: '',
  dateRange: []
})

const newOrder = ref({
  customer_id: null,
  customer: '',
  delivery_date: '',
  items: []
})

const fetchAllItems = async () => {
  try {
    const response = await request.get('/api/items', {
      params: { page: 1, page_size: 1000 }
    })
    allItems.value = response.data?.items || []
  } catch (e) {
    console.error('获取物料列表失败', e)
  }
}

const fetchCustomers = async () => {
  try {
    customerList.value = await request.get('/api/sales-orders/customers')
  } catch (e) {
    console.error('获取客户列表失败', e)
  }
}

const handleCustomerChange = (customerId) => {
  const customer = customerList.value.find(c => c.id === customerId)
  if (customer) {
    newOrder.value.customer = customer.name
  }
}

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
    if (searchForm.value.customer) {
      params.customer = searchForm.value.customer
    }
    if (searchForm.value.dateRange && searchForm.value.dateRange.length === 2) {
      params.start_date = searchForm.value.dateRange[0]
      params.end_date = searchForm.value.dateRange[1]
    }

    orders.value = await request.get('/api/sales-orders', { params })
  } catch (e) {
    console.error('获取数据失败', e)
    ElMessage.error('获取数据失败')
  } finally {
    loading.value = false
  }
}

const fetchStats = async () => {
  try {
    stats.value = await request.get('/api/sales-orders/stats')
  } catch (e) {
    console.error('获取统计数据失败', e)
  }
}

const handleSearch = async () => {
  await fetchOrders()
}

const handleReset = async () => {
  searchForm.value = {
    status: '',
    customer: '',
    dateRange: []
  }
  await fetchOrders()
}

const addItem = () => {
  newOrder.value.items.push({
    item_id: null,
    item_name: '',
    quantity: 1,
    unit_price: 0,
    remark: ''
  })
}

const removeItem = (index) => {
  newOrder.value.items.splice(index, 1)
}

const handleItemChange = (index, itemId) => {
  const selectedItem = allItems.value.find(item => item.id === itemId)
  if (selectedItem) {
    newOrder.value.items[index].item_id = selectedItem.id
    newOrder.value.items[index].item_name = selectedItem.name
  } else {
    newOrder.value.items[index].item_id = null
  }
}

const saveOrder = async () => {
  if (!newOrder.value.customer_id) {
    ElMessage.warning('请选择客户')
    return
  }
  if (!newOrder.value.delivery_date) {
    ElMessage.warning('请选择预计交付日期')
    return
  }
  if (newOrder.value.items.length === 0) {
    ElMessage.warning('请至少添加一条销售明细')
    return
  }

  const hasEmptyItem = newOrder.value.items.some(item => !item.item_id)
  if (hasEmptyItem) {
    ElMessage.warning('请选择所有物品')
    return
  }

  try {
    submitLoading.value = true
    if (isEdit.value) {
      await request.put(`/api/sales-orders/${currentId.value}`, {
        customer_id: newOrder.value.customer_id,
        customer: newOrder.value.customer,
        delivery_date: newOrder.value.delivery_date,
        status: currentOrder.value?.status || '待确认',
        items: newOrder.value.items
      })
      ElMessage.success('更新成功!')
    } else {
      await request.post('/api/sales-orders', newOrder.value)
      ElMessage.success('创建成功!')
    }
    dialogVisible.value = false
    await fetchOrders()
    await fetchStats()
  } catch (error) {
    const errorMsg = error.response?.data?.error || '操作失败'
    ElMessage.error(errorMsg)
  } finally {
    submitLoading.value = false
  }
}

const deleteOrder = async (id, status) => {
  const isForceDelete = status === '已确认' || status === '已完成'
  const confirmMessage = isForceDelete
    ? `确定要强制删除这个${status}的销售订单吗？此操作不可恢复！`
    : '确定要删除这个销售订单吗？'

  try {
    await ElMessageBox.confirm(confirmMessage, isForceDelete ? '警告：强制删除' : '警告', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: isForceDelete ? 'error' : 'warning'
    })

    if (isForceDelete) {
      await request.delete(`/api/sales-orders/${id}/force-delete`)
      ElMessage.success('订单已强制删除')
    } else {
      await request.delete(`/api/sales-orders/${id}`)
      ElMessage.success('订单已删除')
    }
    await fetchOrders()
    await fetchStats()
  } catch (error) {
    if (error !== 'cancel') {
      const errorMsg = error.response?.data?.error || '删除失败'
      ElMessage.error(errorMsg)
    }
  }
}

const handleConfirm = async (id) => {
  try {
    await ElMessageBox.confirm('确定要确认这个销售订单吗？', '提示', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'info'
    })
    await request.post(`/api/sales-orders/${id}/confirm`)
    ElMessage.success('确认成功!')
    await fetchOrders()
    await fetchStats()
  } catch (error) {
    if (error !== 'cancel') {
      const errorMsg = error.response?.data?.error || '确认失败'
      ElMessage.error(errorMsg)
    }
  }
}

const handleComplete = async (id) => {
  try {
    await ElMessageBox.confirm('确定要标记这个销售订单为已完成吗？', '提示', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'info'
    })
    await request.post(`/api/sales-orders/${id}/complete`)
    ElMessage.success('订单已完成')
    await fetchOrders()
    await fetchStats()
  } catch (error) {
    if (error !== 'cancel') {
      const errorMsg = error.response?.data?.error || '操作失败'
      ElMessage.error(errorMsg)
    }
  }
}

const handleCancel = async (id) => {
  try {
    await ElMessageBox.confirm('确定要取消这个销售订单吗？', '提示', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    await request.post(`/api/sales-orders/${id}/cancel`)
    ElMessage.success('订单已取消')
    await fetchOrders()
    await fetchStats()
  } catch (error) {
    if (error !== 'cancel') {
      const errorMsg = error.response?.data?.error || '取消失败'
      ElMessage.error(errorMsg)
    }
  }
}

const handleView = async (row) => {
  try {
    const response = await request.get(`/api/sales-orders/${row.id}`)
    // 兼容多种返回格式，并确保 items 始终是数组
    const data = response.data || response
    currentOrder.value = data.order || data
    currentOrderItems.value = data.items || []
    viewDialogVisible.value = true
  } catch (error) {
    ElMessage.error('获取订单详情失败')
  }
}

const handleEdit = async (row) => {
  try {
    const response = await request.get(`/api/sales-orders/${row.id}`)
    const data = response.data || response
    const order = data.order || data
    const items = data.items || []

    isEdit.value = true
    currentId.value = row.id
    currentOrder.value = order
    newOrder.value = {
      customer_id: order.customer_id,
      customer: order.customer,
      delivery_date: order.delivery_date,
      // 确保映射时变量名正确
      items: items.map(item => ({
        item_id: item.item_id || null,
        item_name: item.item_name || '',
        quantity: item.quantity || 1,
        unit_price: item.unit_price || 0,
        remark: item.remark || ''
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
    customer_id: null,
    customer: '',
    delivery_date: '',
    items: []
  }
  dialogVisible.value = true
}

const getStatusType = (status) => {
  switch (status) {
    case '待确认':
      return 'warning'
    case '已确认':
      return 'success'
    case '已完成':
      return 'info'
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
    fetchAllItems()
    fetchCustomers()
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
  padding: 15px;
  background-color: #f5f7fa;
  border-radius: 4px;
}

.total-amount {
  font-size: 16px;
  color: #606266;
  font-weight: 500;
}

.total-amount .label {
  margin-right: 8px;
}

.amount {
  color: #f56c6c;
  font-size: 24px;
  font-weight: bold;
}

.subtotal {
  color: #67c23a;
  font-weight: 600;
  font-size: 14px;
}

.item-index {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  background-color: #f0f9ff;
  color: #67c23a;
  border-radius: 50%;
  font-weight: bold;
  font-size: 12px;
}

.sales-form :deep(.el-divider__text) {
  display: flex;
  align-items: center;
  font-weight: 600;
  color: #303133;
}

:deep(.el-input-number) {
  width: 100%;
}

:deep(.el-input-number input) {
  text-align: center;
}
</style>