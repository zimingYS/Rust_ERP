<template>
  <div style="padding: 20px;">
    <!-- 关键指标卡片 -->
    <el-row :gutter="20" style="margin-bottom: 20px;">
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card-gradient" style="background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);">
          <div class="stat-content">
            <div class="stat-label" style="color: rgba(255,255,255,0.9);">今日销售</div>
            <div class="stat-value" style="color: white;">¥{{ stats.today_sales?.toFixed(2) || '0.00' }}</div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card-gradient" style="background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);">
          <div class="stat-content">
            <div class="stat-label" style="color: rgba(255,255,255,0.9);">今日采购</div>
            <div class="stat-value" style="color: white;">¥{{ stats.today_purchases?.toFixed(2) || '0.00' }}</div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card-gradient" style="background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%);">
          <div class="stat-content">
            <div class="stat-label" style="color: rgba(255,255,255,0.9);">本月销售</div>
            <div class="stat-value" style="color: white;">¥{{ stats.month_sales?.toFixed(2) || '0.00' }}</div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card-gradient" style="background: linear-gradient(135deg, #43e97b 0%, #38f9d7 100%);">
          <div class="stat-content">
            <div class="stat-label" style="color: rgba(255,255,255,0.9);">待处理订单</div>
            <div class="stat-value" style="color: white;">{{ stats.pending_orders || 0 }}</div>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <!-- 图表区域 -->
    <el-row :gutter="20" style="margin-bottom: 20px;">
      <el-col :span="16">
        <el-card shadow="hover">
          <template #header>
            <div style="display: flex; justify-content: space-between; align-items: center;">
              <span style="font-weight: bold;">销售趋势（最近7天）</span>
            </div>
          </template>
          <v-chart :option="salesTrendOption" style="height: 350px;" autoresize />
        </el-card>
      </el-col>
      <el-col :span="8">
        <el-card shadow="hover">
          <template #header>
            <span style="font-weight: bold;">库存状态分布</span>
          </template>
          <v-chart :option="inventoryPieOption" style="height: 350px;" autoresize />
        </el-card>
      </el-col>
    </el-row>

    <el-row :gutter="20" style="margin-bottom: 20px;">
      <el-col :span="12">
        <el-card shadow="hover">
          <template #header>
            <span style="font-weight: bold;">采购趋势（最近7天）</span>
          </template>
          <v-chart :option="purchaseTrendOption" style="height: 300px;" autoresize />
        </el-card>
      </el-col>
      <el-col :span="12">
        <el-card shadow="hover">
          <template #header>
            <span style="font-weight: bold;">财务收支概况</span>
          </template>
          <v-chart :option="financialPieOption" style="height: 300px;" autoresize />
        </el-card>
      </el-col>
    </el-row>

    <el-row :gutter="20">
      <el-col :span="24">
        <el-card shadow="hover">
          <template #header>
            <span style="font-weight: bold;">热销产品 TOP 10</span>
          </template>
          <v-chart :option="topProductsOption" style="height: 400px;" autoresize />
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script setup>
import { ref, onMounted, computed } from 'vue'
import { useRouter } from 'vue-router'
import request from '../utils/request'
import { ElMessage } from 'element-plus'
import { use } from 'echarts/core'
import { CanvasRenderer } from 'echarts/renderers'
import { BarChart, LineChart, PieChart } from 'echarts/charts'
import {
  TitleComponent,
  TooltipComponent,
  LegendComponent,
  GridComponent
} from 'echarts/components'
import VChart from 'vue-echarts'

use([
  CanvasRenderer,
  BarChart,
  LineChart,
  PieChart,
  TitleComponent,
  TooltipComponent,
  LegendComponent,
  GridComponent
])

const router = useRouter()
const stats = ref({})
const salesTrend = ref([])
const purchaseTrend = ref([])
const inventoryDistribution = ref([])
const topProducts = ref([])
const financialSummary = ref([])

// 销售趋势图配置
const salesTrendOption = computed(() => ({
  tooltip: {
    trigger: 'axis',
    axisPointer: { type: 'shadow' }
  },
  legend: {
    data: ['销售额', '订单数']
  },
  grid: {
    left: '3%',
    right: '4%',
    bottom: '3%',
    containLabel: true
  },
  xAxis: {
    type: 'category',
    data: salesTrend.value.map(item => item.date)
  },
  yAxis: [
    {
      type: 'value',
      name: '销售额(¥)',
      position: 'left'
    },
    {
      type: 'value',
      name: '订单数',
      position: 'right'
    }
  ],
  series: [
    {
      name: '销售额',
      type: 'line',
      smooth: true,
      data: salesTrend.value.map(item => item.amount.toFixed(2)),
      itemStyle: { color: '#667eea' },
      areaStyle: {
        color: {
          type: 'linear',
          x: 0, y: 0, x2: 0, y2: 1,
          colorStops: [
            { offset: 0, color: 'rgba(102, 126, 234, 0.3)' },
            { offset: 1, color: 'rgba(102, 126, 234, 0.05)' }
          ]
        }
      }
    },
    {
      name: '订单数',
      type: 'bar',
      yAxisIndex: 1,
      data: salesTrend.value.map(item => item.order_count),
      itemStyle: { color: '#f093fb' }
    }
  ]
}))

// 采购趋势图配置
const purchaseTrendOption = computed(() => ({
  tooltip: {
    trigger: 'axis',
    axisPointer: { type: 'shadow' }
  },
  grid: {
    left: '3%',
    right: '4%',
    bottom: '3%',
    containLabel: true
  },
  xAxis: {
    type: 'category',
    data: purchaseTrend.value.map(item => item.date)
  },
  yAxis: {
    type: 'value',
    name: '采购额(¥)'
  },
  series: [
    {
      name: '采购额',
      type: 'line',
      smooth: true,
      data: purchaseTrend.value.map(item => item.amount.toFixed(2)),
      itemStyle: { color: '#f5576c' },
      areaStyle: {
        color: {
          type: 'linear',
          x: 0, y: 0, x2: 0, y2: 1,
          colorStops: [
            { offset: 0, color: 'rgba(245, 87, 108, 0.3)' },
            { offset: 1, color: 'rgba(245, 87, 108, 0.05)' }
          ]
        }
      }
    }
  ]
}))

// 库存分布饼图
const inventoryPieOption = computed(() => ({
  tooltip: {
    trigger: 'item',
    formatter: '{b}: {c} ({d}%)'
  },
  legend: {
    orient: 'vertical',
    right: 10,
    top: 'center'
  },
  series: [
    {
      name: '库存状态',
      type: 'pie',
      radius: ['40%', '70%'],
      avoidLabelOverlap: false,
      itemStyle: {
        borderRadius: 10,
        borderColor: '#fff',
        borderWidth: 2
      },
      label: {
        show: true,
        formatter: '{b}\n{c}个'
      },
      emphasis: {
        label: {
          show: true,
          fontSize: 16,
          fontWeight: 'bold'
        }
      },
      data: inventoryDistribution.value.map((item, index) => ({
        value: item.amount,
        name: item.category,
        itemStyle: {
          color: ['#67c23a', '#e6a23c', '#f56c6c'][index]
        }
      }))
    }
  ]
}))

// 财务收支饼图
const financialPieOption = computed(() => ({
  tooltip: {
    trigger: 'item',
    formatter: '{b}: ¥{c} ({d}%)'
  },
  legend: {
    orient: 'vertical',
    right: 10,
    top: 'center'
  },
  series: [
    {
      name: '财务概况',
      type: 'pie',
      radius: ['40%', '70%'],
      avoidLabelOverlap: false,
      itemStyle: {
        borderRadius: 10,
        borderColor: '#fff',
        borderWidth: 2
      },
      label: {
        show: true,
        formatter: '{b}\n¥{c}'
      },
      data: financialSummary.value.map((item, index) => ({
        value: item.amount.toFixed(2),
        name: item.category,
        itemStyle: {
          color: ['#67c23a', '#f56c6c'][index]
        }
      }))
    }
  ]
}))

// 热销产品柱状图
const topProductsOption = computed(() => ({
  tooltip: {
    trigger: 'axis',
    axisPointer: { type: 'shadow' },
    formatter: (params) => {
      const data = params[0]
      return `${data.name}<br/>销量: ${data.value}件<br/>金额: ¥${topProducts.value[data.dataIndex]?.total_amount?.toFixed(2) || '0.00'}`
    }
  },
  grid: {
    left: '3%',
    right: '4%',
    bottom: '3%',
    containLabel: true
  },
  xAxis: {
    type: 'value',
    name: '销量(件)'
  },
  yAxis: {
    type: 'category',
    data: [...topProducts.value].reverse().map(item => item.item_name),
    axisLabel: {
      interval: 0,
      formatter: (value) => {
        return value.length > 10 ? value.substring(0, 10) + '...' : value
      }
    }
  },
  series: [
    {
      name: '销量',
      type: 'bar',
      data: [...topProducts.value].reverse().map(item => Number(item.total_quantity)),
      itemStyle: {
        color: {
          type: 'linear',
          x: 0, y: 0, x2: 1, y2: 0,
          colorStops: [
            { offset: 0, color: '#667eea' },
            { offset: 1, color: '#764ba2' }
          ]
        },
        borderRadius: [0, 5, 5, 0]
      }
    }
  ]
}))

const fetchDashboardStats = async () => {
  try {
    stats.value = await request.get('/api/dashboard/stats')
  } catch (e) {
    console.error('获取仪表盘统计数据失败', e)
  }
}

const fetchSalesTrend = async () => {
  try {
    salesTrend.value = await request.get('/api/dashboard/sales-trend')
  } catch (e) {
    console.error('获取销售趋势失败', e)
  }
}

const fetchPurchaseTrend = async () => {
  try {
    purchaseTrend.value = await request.get('/api/dashboard/purchase-trend')
  } catch (e) {
    console.error('获取采购趋势失败', e)
  }
}

const fetchInventoryDistribution = async () => {
  try {
    inventoryDistribution.value = await request.get('/api/dashboard/inventory-distribution')
  } catch (e) {
    console.error('获取库存分布失败', e)
  }
}

const fetchTopProducts = async () => {
  try {
    topProducts.value = await request.get('/api/dashboard/top-products')
  } catch (e) {
    console.error('获取热销产品失败', e)
  }
}

const fetchFinancialSummary = async () => {
  try {
    financialSummary.value = await request.get('/api/dashboard/financial-summary')
  } catch (e) {
    console.error('获取财务概况失败', e)
  }
}

onMounted(() => {
  const token = localStorage.getItem('erp_token')
  if (!token) {
    router.push('/login')
  } else {
    fetchDashboardStats()
    fetchSalesTrend()
    fetchPurchaseTrend()
    fetchInventoryDistribution()
    fetchTopProducts()
    fetchFinancialSummary()
  }
})
</script>

<style scoped>
.stat-content {
  text-align: center;
  padding: 10px 0;
}

.stat-label {
  font-size: 14px;
  margin-bottom: 10px;
}

.stat-value {
  font-size: 28px;
  font-weight: bold;
}

.stat-card-gradient {
  transition: transform 0.3s, box-shadow 0.3s;
}

.stat-card-gradient:hover {
  transform: translateY(-5px);
  box-shadow: 0 8px 16px rgba(0, 0, 0, 0.2);
}
</style>