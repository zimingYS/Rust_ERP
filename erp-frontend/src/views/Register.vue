<template>
  <div style="min-height: 100vh; display: flex; justify-content: center; align-items: center; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);">
    <div style="background: white; padding: 40px; border-radius: 10px; box-shadow: 0 10px 40px rgba(0,0,0,0.2); width: 400px;">
      <h2 style="text-align: center; color: #333; margin-bottom: 30px;">用户注册</h2>

      <el-form :model="registerForm" :rules="rules" ref="registerFormRef" label-width="80px">
        <el-form-item label="用户名" prop="username">
          <el-input
              v-model="registerForm.username"
              placeholder="请输入用户名"
              prefix-icon="User"
          />
        </el-form-item>

        <el-form-item label="密码" prop="password">
          <el-input
              v-model="registerForm.password"
              type="password"
              placeholder="请输入密码（至少6位）"
              show-password
              prefix-icon="Lock"
          />
        </el-form-item>

        <el-form-item label="确认密码" prop="confirmPassword">
          <el-input
              v-model="registerForm.confirmPassword"
              type="password"
              placeholder="请再次输入密码"
              show-password
              prefix-icon="Lock"
          />
        </el-form-item>

        <el-alert
            title="提示"
            type="info"
            description="新注册的用户默认为普通用户权限，如需管理员权限请联系系统管理员。"
            :closable="false"            style="margin-bottom: 20px;"
        />

        <el-form-item>
          <el-button
              type="primary"
              @click="handleRegister"
              :loading="loading"            style="width: 100%;"
              size="large"
          >
            注册
          </el-button>
        </el-form-item>

        <el-form-item>
          <div style="text-align: center; width: 100%;">
            <span style="color: #666;">已有账号？</span>
            <el-link type="primary" @click="goToLogin" style="margin-left: 5px;">
              立即登录
            </el-link>
          </div>
        </el-form-item>
      </el-form>
    </div>
  </div>
</template>

<script setup>import { ref, reactive } from 'vue'
import { useRouter } from 'vue-router'
import request from '../utils/request'
import { ElMessage } from 'element-plus'

const router = useRouter()
const registerFormRef = ref(null)
const loading = ref(false)

const registerForm = reactive({
  username: '',
  password: '',
  confirmPassword: '',
})

const validatePass = (rule, value, callback) => {
  if (value === '') {
    callback(new Error('请输入密码'))
  } else if (value.length < 6) {
    callback(new Error('密码长度不能少于6位'))
  } else {
    if (registerForm.confirmPassword !== '') {
      registerFormRef.value?.validateField('confirmPassword')
    }
    callback()
  }
}

const validatePass2 = (rule, value, callback) => {
  if (value === '') {
    callback(new Error('请再次输入密码'))
  } else if (value !== registerForm.password) {
    callback(new Error('两次输入密码不一致!'))
  } else {
    callback()
  }
}

const rules = {
  username: [
    { required: true, message: '请输入用户名', trigger: 'blur' },
    { min: 3, max: 20, message: '用户名长度在 3 到 20 个字符', trigger: 'blur' }
  ],
  password: [
    { validator: validatePass, trigger: 'blur' }
  ],
  confirmPassword: [
    { validator: validatePass2, trigger: 'blur' }
  ]
}

const handleRegister = async () => {
  if (!registerFormRef.value) return

  await registerFormRef.value.validate(async (valid) => {
    if (valid) {
      loading.value = true
      try {
        const { confirmPassword, ...registerData } = registerForm
        await request.post('/api/register', registerData)
        ElMessage.success('注册成功！即将跳转到登录页')

        setTimeout(() => {
          router.push('/login')
        }, 1500)
      } catch (error) {
        // 错误已在拦截器中处理
      } finally {
        loading.value = false
      }
    } else {
      ElMessage.warning('请填写完整的注册信息')
      return false
    }
  })
}

const goToLogin = () => {
  router.push('/login')
}
</script>

<style scoped>.el-input-number {
  width: 100%;
}
</style>