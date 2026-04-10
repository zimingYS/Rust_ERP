<template>
  <div style="padding: 20px;">
    <el-button type="primary" @click="openAddDialog" style="margin-bottom: 20px;">新增物料</el-button>

    <el-table :data="items" border style="width: 100%">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="name" label="物料名称" />
      <el-table-column prop="sku" label="SKU 编号" />
      <el-table-column prop="stock" label="当前库存" />
      <el-table-column label="操作" width="180">
        <template #default="scope">
          <div style="display: flex; gap: 8px;">
            <el-button size="small" @click="handleEdit(scope.row)">编辑</el-button>
            <el-button type="danger" size="small" @click="deleteItem(scope.row.id)">删除</el-button>
          </div>
        </template>
      </el-table-column>
    </el-table>

    <el-dialog v-model="dialogVisible" :title="isEdit ? '编辑物料' : '添加新物料'" width="30%">
      <el-form :model="newItem">
        <el-form-item label="名称"><el-input v-model="newItem.name" /></el-form-item>
        <el-form-item label="SKU"><el-input v-model="newItem.sku" /></el-form-item>
        <el-form-item label="库存"><el-input-number v-model="newItem.stock" :min="0" /></el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" @click="saveItem">确认提交</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import request from '../utils/request'
import { ElMessage } from 'element-plus'

const router = useRouter()
const items = ref([])
const dialogVisible = ref(false)
const newItem = ref({ name: '', sku: '', stock: 0 })
const isEdit = ref(false)
const currentId = ref(null)

const fetchItems = async () => {
  try {
    items.value = await request.get('/api/items')
  } catch (e) {
    console.error('获取数据失败', e)
  }
}

const saveItem = async () => {
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
  }
}

const deleteItem = async (id) => {
  try {
    await request.delete(`/api/items/${id}`)
    ElMessage.success('物料已成功删除')
    fetchItems()
  } catch (error) {
    const errorMsg = error.response?.data?.message || '删除失败'
    ElMessage.error(errorMsg)
  }
}

const handleEdit = (row) => {
  isEdit.value = true
  currentId.value = row.id
  newItem.value = { ...row }
  dialogVisible.value = true
}

const openAddDialog = () => {
  isEdit.value = false
  newItem.value = { name: '', sku: '', stock: 0 }
  dialogVisible.value = true
}

const handleLogout = () => {
  localStorage.removeItem('erp_token')
  router.push('/login')
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