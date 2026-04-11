<template>
  <div style="padding: 20px;">
    <el-button type="primary" @click="openAddDialog" style="margin-bottom: 20px;">新建销售订单</el-button>

    <el-table :data="orders" border style="width: 100%">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="order_number" label="订单编号" width="180" />
      <el-table-column prop="customer" label="客户" />
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
      <el-table-column label="操作" width="280">
        <template #default="scope">
          <div style="display: flex; gap: 8px;">
            <el-button size="small" @click="handleView(scope.row)">查看</el-button>
            <el-button size="small" @click="handleEdit(scope.row)">编辑</el-button>
            <el-button
                v-if="scope.row.status === '待确认'"
                type="success"
                size="small"
                @click="handleConfirm(scope.row.id)"
            >
              确认
            </el-button>
            <el-button type="danger" size="small" @click="deleteOrder(scope.row.id)">删除</el-button>
          </div>
        </template>
      </el-table-column>
    </el-table>

    <el-dialog v-model="dialogVisible" :title="isEdit ? '编辑销售订单' : '新建销售订单'" width="70%">
      <el-form :model="newOrder" label-width="120px">
        <el-form-item label="客户">
          <el-input v-model="newOrder.customer" placeholder="请输入客户名称" />
        </el-form-item>
        <el-form-item label="预计交付日期">
          <el-date-picker
              v-model="newOrder.delivery_date"
              type="date"
              placeholder="选择日期"              style="width: 100%;"
              value-format="YYYY-MM-DD"
          />
        </el-form-item>
        <el-form-item label="销售明细">
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
                      controls-position="right"                    style="width: 100%;"
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
                      controls-position="right"                    style="width: 100%;"
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

<script setup>import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import request from '../utils/request'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Plus } from '@element-plus/icons-vue'

const router = useRouter()
const orders = ref([])
const dialogVisible = ref(false)
const viewDialogVisible = ref(false)
const isEdit = ref(false)
const currentId = ref(null)
const currentOrder = ref(null)
const currentOrderItems = ref([])

const newOrder = ref({
  customer: '',
  delivery_date: '',
  items: []
})

const calculateTotalAmount = computed(() => {
  return newOrder.value.items.reduce((total, item) => {
    return total + (item.quantity * item.unit_price)
  }, 0)
})

const fetchOrders = async () => {
  try {
    orders.value = await request.get('/api/sales-orders')
  } catch (e) {
    console.error('获取数据失败', e)
  }
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
  if (!newOrder.value.customer) {
    ElMessage.warning('请输入客户名称')
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

  try {
    if (isEdit.value) {
      await request.put(`/api/sales-orders/${currentId.value}`, {
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
    fetchOrders()
  } catch (error) {
    const errorMsg = error.response?.data?.error || '操作失败'
    ElMessage.error(errorMsg)
  }
}

const deleteOrder = async (id) => {
  try {
    await ElMessageBox.confirm('确定要删除这个销售订单吗？', '警告', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    await request.delete(`/api/sales-orders/${id}`)
    ElMessage.success('订单已删除')
    fetchOrders()
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
    fetchOrders()
  } catch (error) {
    if (error !== 'cancel') {
      const errorMsg = error.response?.data?.error || '确认失败'
      ElMessage.error(errorMsg)
    }
  }
}

const handleView = async (row) => {
  try {
    const [order, items] = await request.get(`/api/sales-orders/${row.id}`)
    currentOrder.value = order
    currentOrderItems.value = items
    viewDialogVisible.value = true
  } catch (error) {
    ElMessage.error('获取订单详情失败')
  }
}

const handleEdit = async (row) => {
  try {
    const [order, items] = await request.get(`/api/sales-orders/${row.id}`)
    isEdit.value = true
    currentId.value = row.id
    currentOrder.value = order
    newOrder.value = {
      customer: order.customer,
      delivery_date: order.delivery_date,
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
  }
})
</script>

<style scoped>.items-container {
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