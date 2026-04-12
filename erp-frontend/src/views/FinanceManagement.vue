<template>
  <div style="padding: 20px;">
    <!-- 统计卡片 -->
    <el-row :gutter="20" style="margin-bottom: 20px;">
      <el-col :span="6">
        <el-card shadow="hover">
          <div class="stat-card">
            <div class="stat-icon" style="background-color: #67c23a;">
              <el-icon :size="30"><TrendCharts /></el-icon>
            </div>
            <div class="stat-content">
              <div class="stat-value">¥{{ stats.total_income.toFixed(2) }}</div>
              <div class="stat-label">总收入</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover">
          <div class="stat-card">
            <div class="stat-icon" style="background-color: #f56c6c;">
              <el-icon :size="30"><ShoppingCart /></el-icon>
            </div>
            <div class="stat-content">
              <div class="stat-value">¥{{ stats.total_expense.toFixed(2) }}</div>
              <div class="stat-label">总支出</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover">
          <div class="stat-card">
            <div class="stat-icon" :style="{ backgroundColor: stats.net_profit >= 0 ? '#409eff' : '#f56c6c' }">
              <el-icon :size="30"><Coin /></el-icon>
            </div>
            <div class="stat-content">
              <div class="stat-value">¥{{ stats.net_profit.toFixed(2) }}</div>
              <div class="stat-label">净利润</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover">
          <div class="stat-card">
            <div class="stat-icon" style="background-color: #e6a23c;">
              <el-icon :size="30"><Document /></el-icon>
            </div>
            <div class="stat-content">
              <div class="stat-value">{{ stats.transaction_count }}</div>
              <div class="stat-label">交易笔数</div>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <!-- 本月概况 -->
    <el-card style="margin-bottom: 20px;">
      <template #header>
        <div style="display: flex; justify-content: space-between; align-items: center;">
          <span style="font-weight: bold;">本月财务概况</span>
        </div>
      </template>
      <el-row :gutter="20">
        <el-col :span="8">
          <div class="month-stat">
            <div class="label">本月收入</div>
            <div class="value income">¥{{ stats.this_month_income.toFixed(2) }}</div>
          </div>
        </el-col>
        <el-col :span="8">
          <div class="month-stat">
            <div class="label">本月支出</div>
            <div class="value expense">¥{{ stats.this_month_expense.toFixed(2) }}</div>
          </div>
        </el-col>
        <el-col :span="8">
          <div class="month-stat">
            <div class="label">本月利润</div>
            <div class="value" :style="{ color: stats.this_month_profit >= 0 ? '#67c23a' : '#f56c6c' }">
              ¥{{ stats.this_month_profit.toFixed(2) }}
            </div>
          </div>
        </el-col>
      </el-row>
    </el-card>

    <!-- 搜索和筛选 -->
    <el-card style="margin-bottom: 20px;">
      <div style="display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 10px;">
        <el-form :inline="true" :model="searchForm" style="margin: 0;">
          <el-form-item label="交易类型" style="margin-bottom: 0;">
            <el-select v-model="searchForm.transaction_type" placeholder="请选择类型" clearable style="width: 150px;">
              <el-option label="收入" value="收入" />
              <el-option label="支出" value="支出" />
            </el-select>
          </el-form-item>
          <el-form-item label="分类" style="margin-bottom: 0;">
            <el-input v-model="searchForm.category" placeholder="请输入分类" clearable style="width: 180px;" />
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
          <el-button type="success" @click="openAddDialog">
            <el-icon><Plus /></el-icon>
            新增交易
          </el-button>
        </div>
      </div>
    </el-card>

    <!-- 交易表格 -->
    <el-table :data="transactions" border style="width: 100%" v-loading="loading">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="transaction_type" label="类型" width="100">
        <template #default="scope">
          <el-tag :type="scope.row.transaction_type === '收入' ? 'success' : 'danger'">
            {{ scope.row.transaction_type }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column prop="category" label="分类" width="120" />
      <el-table-column prop="amount" label="金额" width="150">
        <template #default="scope">
          <span :style="{ color: scope.row.transaction_type === '收入' ? '#67c23a' : '#f56c6c', fontWeight: 'bold' }">
            {{ scope.row.transaction_type === '收入' ? '+' : '-' }}¥{{ scope.row.amount.toFixed(2) }}
          </span>
        </template>
      </el-table-column>
      <el-table-column prop="description" label="描述" min-width="200" show-overflow-tooltip />
      <el-table-column prop="transaction_date" label="交易日期" width="180" />
      <el-table-column label="操作" width="150" fixed="right">
        <template #default="scope">
          <el-button
            type="danger"
            size="small"
            @click="deleteTransaction(scope.row.id)"
          >
            删除
          </el-button>
        </template>
      </el-table-column>
    </el-table>

    <!-- 新增交易对话框 -->
    <el-dialog v-model="dialogVisible" title="新增财务交易" width="600px">
      <el-form :model="newTransaction" label-width="120px">
        <el-form-item label="交易类型" required>
          <el-radio-group v-model="newTransaction.transaction_type">
            <el-radio label="收入">收入</el-radio>
            <el-radio label="支出">支出</el-radio>
          </el-radio-group>
        </el-form-item>
        <el-form-item label="分类" required>
          <el-input v-model="newTransaction.category" placeholder="例如：销售收入、采购成本、办公费用等" />
        </el-form-item>
        <el-form-item label="金额" required>
          <el-input-number
            v-model="newTransaction.amount"
            :min="0.01"
            :precision="2"
            :step="100"
            controls-position="right"
            style="width: 100%;"
          />
        </el-form-item>
        <el-form-item label="交易日期">
          <el-date-picker
            v-model="newTransaction.transaction_date"
            type="date"
            placeholder="选择日期"
            style="width: 100%;"
            value-format="YYYY-MM-DD"
          />
        </el-form-item>
        <el-form-item label="描述">
          <el-input
            v-model="newTransaction.description"
            type="textarea"
            :rows="3"
            placeholder="请输入交易描述（可选）"
          />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" @click="saveTransaction">确认提交</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import request from '../utils/request'
import { ElMessage, ElMessageBox } from 'element-plus'
import { TrendCharts, ShoppingCart, Coin, Document, Search, Refresh, Plus } from '@element-plus/icons-vue'

const router = useRouter()
const transactions = ref([])
const loading = ref(false)
const dialogVisible = ref(false)

const stats = ref({
  total_income: 0,
  total_expense: 0,
  net_profit: 0,
  this_month_income: 0,
  this_month_expense: 0,
  this_month_profit: 0,
  transaction_count: 0
})

const searchForm = ref({
  transaction_type: '',
  category: '',
  dateRange: []
})

const newTransaction = ref({
  transaction_type: '收入',
  category: '',
  amount: 0,
  description: '',
  transaction_date: ''
})

const fetchTransactions = async () => {
  loading.value = true
  try {
    const params = {}
    if (searchForm.value.transaction_type) {
      params.transaction_type = searchForm.value.transaction_type
    }
    if (searchForm.value.category) {
      params.category = searchForm.value.category
    }
    if (searchForm.value.dateRange && searchForm.value.dateRange.length === 2) {
      params.start_date = searchForm.value.dateRange[0]
      params.end_date = searchForm.value.dateRange[1]
    }

    transactions.value = await request.get('/api/finance/transactions', { params })
  } catch (e) {
    console.error('获取交易数据失败', e)
    ElMessage.error('获取交易数据失败')
  } finally {
    loading.value = false
  }
}

const fetchStats = async () => {
  try {
    stats.value = await request.get('/api/finance/stats')
  } catch (e) {
    console.error('获取统计数据失败', e)
  }
}

const handleSearch = () => {
  fetchTransactions()
}

const handleReset = () => {
  searchForm.value = {
    transaction_type: '',
    category: '',
    dateRange: []
  }
  fetchTransactions()
}

const openAddDialog = () => {
  newTransaction.value = {
    transaction_type: '收入',
    category: '',
    amount: 0,
    description: '',
    transaction_date: new Date().toISOString().split('T')[0]
  }
  dialogVisible.value = true
}

const saveTransaction = async () => {
  if (!newTransaction.value.category) {
    ElMessage.warning('请输入分类')
    return
  }
  if (newTransaction.value.amount <= 0) {
    ElMessage.warning('金额必须大于0')
    return
  }

  try {
    await request.post('/api/finance/transactions', newTransaction.value)
    ElMessage.success('创建成功!')
    dialogVisible.value = false
    fetchTransactions()
    fetchStats()
  } catch (error) {
    const errorMsg = error.response?.data?.error || '操作失败'
    ElMessage.error(errorMsg)
  }
}

const deleteTransaction = async (id) => {
  try {
    await ElMessageBox.confirm('确定要删除这条交易记录吗？', '警告', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    await request.delete(`/api/finance/transactions/${id}`)
    ElMessage.success('删除成功')
    fetchTransactions()
    fetchStats()
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
    fetchTransactions()
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

.month-stat {
  text-align: center;
  padding: 20px;
}

.month-stat .label {
  font-size: 14px;
  color: #909399;
  margin-bottom: 10px;
}

.month-stat .value {
  font-size: 28px;
  font-weight: bold;
}

.income {
  color: #67c23a;
}

.expense {
  color: #f56c6c;
}
</style>
