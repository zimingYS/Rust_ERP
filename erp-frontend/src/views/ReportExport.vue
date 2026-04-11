<template>
  <div style="padding: 20px;">
    <el-card>
      <template #header>
        <div style="font-weight: bold; font-size: 18px;">报表导出中心</div>
      </template>

      <el-row :gutter="20">
        <!-- 采购订单报表 -->
        <el-col :span="12">
          <el-card shadow="hover" class="report-card">
            <div class="report-icon" style="background-color: #409eff;">
              <el-icon :size="40"><ShoppingCart /></el-icon>
            </div>
            <h3>采购订单报表</h3>
            <p>导出所有采购订单数据，支持按日期范围筛选</p>
            <el-form :model="purchaseForm" label-width="100px" style="margin-top: 20px;">
              <el-form-item label="开始日期">
                <el-date-picker
                  v-model="purchaseForm.start_date"
                  type="date"
                  placeholder="选择日期"
                  value-format="YYYY-MM-DD"
                  style="width: 100%;"
                />
              </el-form-item>
              <el-form-item label="结束日期">
                <el-date-picker
                  v-model="purchaseForm.end_date"
                  type="date"
                  placeholder="选择日期"
                  value-format="YYYY-MM-DD"
                  style="width: 100%;"
                />
              </el-form-item>
              <el-form-item label="导出格式">
                <el-radio-group v-model="purchaseForm.format">
                  <el-radio label="csv">CSV</el-radio>
                  <el-radio label="json">JSON</el-radio>
                </el-radio-group>
              </el-form-item>
              <el-form-item>
                <el-button type="primary" @click="exportPurchaseOrders" :loading="exporting.purchase">
                  <el-icon><Download /></el-icon>
                  导出报表
                </el-button>
              </el-form-item>
            </el-form>
          </el-card>
        </el-col>

        <!-- 销售订单报表 -->
        <el-col :span="12">
          <el-card shadow="hover" class="report-card">
            <div class="report-icon" style="background-color: #67c23a;">
              <el-icon :size="40"><Sell /></el-icon>
            </div>
            <h3>销售订单报表</h3>
            <p>导出所有销售订单数据，支持按日期范围筛选</p>
            <el-form :model="salesForm" label-width="100px" style="margin-top: 20px;">
              <el-form-item label="开始日期">
                <el-date-picker
                  v-model="salesForm.start_date"
                  type="date"
                  placeholder="选择日期"
                  value-format="YYYY-MM-DD"
                  style="width: 100%;"
                />
              </el-form-item>
              <el-form-item label="结束日期">
                <el-date-picker
                  v-model="salesForm.end_date"
                  type="date"
                  placeholder="选择日期"
                  value-format="YYYY-MM-DD"
                  style="width: 100%;"
                />
              </el-form-item>
              <el-form-item label="导出格式">
                <el-radio-group v-model="salesForm.format">
                  <el-radio label="csv">CSV</el-radio>
                  <el-radio label="json">JSON</el-radio>
                </el-radio-group>
              </el-form-item>
              <el-form-item>
                <el-button type="success" @click="exportSalesOrders" :loading="exporting.sales">
                  <el-icon><Download /></el-icon>
                  导出报表
                </el-button>
              </el-form-item>
            </el-form>
          </el-card>
        </el-col>
      </el-row>

      <el-row :gutter="20" style="margin-top: 20px;">
        <!-- 库存报表 -->
        <el-col :span="8">
          <el-card shadow="hover" class="report-card">
            <div class="report-icon" style="background-color: #e6a23c;">
              <el-icon :size="40"><Box /></el-icon>
            </div>
            <h3>库存报表</h3>
            <p>导出当前所有物料的库存信息</p>
            <el-button type="warning" @click="exportInventory" :loading="exporting.inventory" style="margin-top: 20px; width: 100%;">
              <el-icon><Download /></el-icon>
              导出CSV
            </el-button>
          </el-card>
        </el-col>

        <!-- 财务报表 -->
        <el-col :span="8">
          <el-card shadow="hover" class="report-card">
            <div class="report-icon" style="background-color: #f56c6c;">
              <el-icon :size="40"><Coin /></el-icon>
            </div>
            <h3>财务报表</h3>
            <p>导出所有财务交易记录</p>
            <el-form :model="financeForm" label-width="100px" style="margin-top: 20px;">
              <el-form-item label="开始日期">
                <el-date-picker
                  v-model="financeForm.start_date"
                  type="date"
                  placeholder="选择日期"
                  value-format="YYYY-MM-DD"
                  style="width: 100%;"
                />
              </el-form-item>
              <el-form-item label="结束日期">
                <el-date-picker
                  v-model="financeForm.end_date"
                  type="date"
                  placeholder="选择日期"
                  value-format="YYYY-MM-DD"
                  style="width: 100%;"
                />
              </el-form-item>
              <el-form-item>
                <el-button type="danger" @click="exportFinance" :loading="exporting.finance" style="width: 100%;">
                  <el-icon><Download /></el-icon>
                  导出CSV
                </el-button>
              </el-form-item>
            </el-form>
          </el-card>
        </el-col>

        <!-- 审计日志报表 -->
        <el-col :span="8">
          <el-card shadow="hover" class="report-card">
            <div class="report-icon" style="background-color: #909399;">
              <el-icon :size="40"><Document /></el-icon>
            </div>
            <h3>审计日志报表</h3>
            <p>导出系统操作日志记录</p>
            <el-form :model="auditForm" label-width="100px" style="margin-top: 20px;">
              <el-form-item label="开始日期">
                <el-date-picker
                  v-model="auditForm.start_date"
                  type="date"
                  placeholder="选择日期"
                  value-format="YYYY-MM-DD"
                  style="width: 100%;"
                />
              </el-form-item>
              <el-form-item label="结束日期">
                <el-date-picker
                  v-model="auditForm.end_date"
                  type="date"
                  placeholder="选择日期"
                  value-format="YYYY-MM-DD"
                  style="width: 100%;"
                />
              </el-form-item>
              <el-form-item>
                <el-button @click="exportAuditLogs" :loading="exporting.audit" style="width: 100%;">
                  <el-icon><Download /></el-icon>
                  导出CSV
                </el-button>
              </el-form-item>
            </el-form>
          </el-card>
        </el-col>
      </el-row>
    </el-card>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { ShoppingCart, Sell, Box, Coin, Document, Download } from '@element-plus/icons-vue'

const router = useRouter()

const exporting = ref({
  purchase: false,
  sales: false,
  inventory: false,
  finance: false,
  audit: false
})

const purchaseForm = ref({
  start_date: '',
  end_date: '',
  format: 'csv'
})

const salesForm = ref({
  start_date: '',
  end_date: '',
  format: 'csv'
})

const financeForm = ref({
  start_date: '',
  end_date: ''
})

const auditForm = ref({
  start_date: '',
  end_date: ''
})

const downloadFile = (blob, filename) => {
  const url = window.URL.createObjectURL(blob)
  const link = document.createElement('a')
  link.href = url
  link.setAttribute('download', filename)
  document.body.appendChild(link)
  link.click()
  link.remove()
  window.URL.revokeObjectURL(url)
}

const exportPurchaseOrders = async () => {
  exporting.value.purchase = true
  try {
    const params = new URLSearchParams()
    if (purchaseForm.value.start_date) {
      params.append('start_date', purchaseForm.value.start_date)
    }
    if (purchaseForm.value.end_date) {
      params.append('end_date', purchaseForm.value.end_date)
    }
    params.append('format', purchaseForm.value.format)

    const response = await fetch(`/api/reports/export/purchase-orders?${params.toString()}`, {
      headers: {
        'Authorization': `Bearer ${localStorage.getItem('erp_token')}`
      }
    })

    if (!response.ok) {
      throw new Error('导出失败')
    }

    const blob = await response.blob()
    const filename = response.headers.get('content-disposition')?.split('filename=')[1]?.replace(/"/g, '') || 'purchase_orders.csv'
    downloadFile(blob, filename)
    ElMessage.success('导出成功')
  } catch (error) {
    ElMessage.error('导出失败: ' + error.message)
  } finally {
    exporting.value.purchase = false
  }
}

const exportSalesOrders = async () => {
  exporting.value.sales = true
  try {
    const params = new URLSearchParams()
    if (salesForm.value.start_date) {
      params.append('start_date', salesForm.value.start_date)
    }
    if (salesForm.value.end_date) {
      params.append('end_date', salesForm.value.end_date)
    }
    params.append('format', salesForm.value.format)

    const response = await fetch(`/api/reports/export/sales-orders?${params.toString()}`, {
      headers: {
        'Authorization': `Bearer ${localStorage.getItem('erp_token')}`
      }
    })

    if (!response.ok) {
      throw new Error('导出失败')
    }

    const blob = await response.blob()
    const filename = response.headers.get('content-disposition')?.split('filename=')[1]?.replace(/"/g, '') || 'sales_orders.csv'
    downloadFile(blob, filename)
    ElMessage.success('导出成功')
  } catch (error) {
    ElMessage.error('导出失败: ' + error.message)
  } finally {
    exporting.value.sales = false
  }
}

const exportInventory = async () => {
  exporting.value.inventory = true
  try {
    const response = await fetch('/api/reports/export/inventory', {
      headers: {
        'Authorization': `Bearer ${localStorage.getItem('erp_token')}`
      }
    })

    if (!response.ok) {
      throw new Error('导出失败')
    }

    const blob = await response.blob()
    const filename = response.headers.get('content-disposition')?.split('filename=')[1]?.replace(/"/g, '') || 'inventory.csv'
    downloadFile(blob, filename)
    ElMessage.success('导出成功')
  } catch (error) {
    ElMessage.error('导出失败: ' + error.message)
  } finally {
    exporting.value.inventory = false
  }
}

const exportFinance = async () => {
  exporting.value.finance = true
  try {
    const params = new URLSearchParams()
    if (financeForm.value.start_date) {
      params.append('start_date', financeForm.value.start_date)
    }
    if (financeForm.value.end_date) {
      params.append('end_date', financeForm.value.end_date)
    }

    const response = await fetch(`/api/reports/export/financial-transactions?${params.toString()}`, {
      headers: {
        'Authorization': `Bearer ${localStorage.getItem('erp_token')}`
      }
    })

    if (!response.ok) {
      throw new Error('导出失败')
    }

    const blob = await response.blob()
    const filename = response.headers.get('content-disposition')?.split('filename=')[1]?.replace(/"/g, '') || 'financial_transactions.csv'
    downloadFile(blob, filename)
    ElMessage.success('导出成功')
  } catch (error) {
    ElMessage.error('导出失败: ' + error.message)
  } finally {
    exporting.value.finance = false
  }
}

const exportAuditLogs = async () => {
  exporting.value.audit = true
  try {
    const params = new URLSearchParams()
    if (auditForm.value.start_date) {
      params.append('start_date', auditForm.value.start_date)
    }
    if (auditForm.value.end_date) {
      params.append('end_date', auditForm.value.end_date)
    }

    const response = await fetch(`/api/reports/export/audit-logs?${params.toString()}`, {
      headers: {
        'Authorization': `Bearer ${localStorage.getItem('erp_token')}`
      }
    })

    if (!response.ok) {
      throw new Error('导出失败')
    }

    const blob = await response.blob()
    const filename = response.headers.get('content-disposition')?.split('filename=')[1]?.replace(/"/g, '') || 'audit_logs.csv'
    downloadFile(blob, filename)
    ElMessage.success('导出成功')
  } catch (error) {
    ElMessage.error('导出失败: ' + error.message)
  } finally {
    exporting.value.audit = false
  }
}
</script>

<style scoped>
.report-card {
  height: 100%;
  transition: transform 0.3s;
}

.report-card:hover {
  transform: translateY(-5px);
}

.report-icon {
  width: 80px;
  height: 80px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  margin: 0 auto 20px;
}

.report-card h3 {
  text-align: center;
  margin-bottom: 10px;
  color: #303133;
}

.report-card p {
  text-align: center;
  color: #909399;
  font-size: 14px;
}
</style>
