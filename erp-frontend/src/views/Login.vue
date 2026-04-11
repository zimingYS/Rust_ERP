<template>
  <div class="login-container">
    <div class="login-box">
      <h2 style="text-align: center; color: #333; margin-bottom: 30px;">ERP 系统登录</h2>

      <el-form :model="loginForm" :rules="rules" ref="loginFormRef"  class="login-form">
        <el-form-item label="账号" prop="username">
          <el-input
              v-model="loginForm.username"
              placeholder="请输入账号"
              prefix-icon="User"
              @keyup.enter="handleLogin"
          />
        </el-form-item>

        <el-form-item label="密码" prop="password">
          <el-input
              v-model="loginForm.password"
              type="password"
              placeholder="请输入密码"
              show-password
              prefix-icon="Lock"
              @keyup.enter="handleLogin"
          />
        </el-form-item>

        <el-form-item>
          <el-button
              type="primary"
              @click="handleLogin"
              :loading="loading"              style="width: 100%;"
              size="large"
          >
            登录
          </el-button>
        </el-form-item>

        <el-form-item>
          <div style="text-align: center; width: 100%;">
            <span style="color: #666;">还没有账号？</span>
            <el-link type="primary" @click="goToRegister" style="margin-left: 5px;">
              立即注册
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
const loginFormRef = ref(null)
const loading = ref(false)

const loginForm = reactive({
  username: '',
  password: ''
})

const rules = {
  username: [
    { required: true, message: '请输入账号', trigger: 'blur' }
  ],
  password: [
    { required: true, message: '请输入密码', trigger: 'blur' }
  ]
}

const handleLogin = async () => {
  if (!loginFormRef.value) return

  await loginFormRef.value.validate(async (valid) => {
    if (valid) {
      loading.value = true
      try {
        const loginData = {
          username: loginForm.username,
          password: loginForm.password
        }
        const res = await request.post('/api/login', loginData)
        if (res.token) {
          localStorage.setItem('erp_token', res.token)

          try {
            const tokenParts = res.token.split('.')
            if (tokenParts.length === 3) {
              const payload = JSON.parse(atob(tokenParts[1]))
              localStorage.setItem('user_role', payload.role_id.toString())
              localStorage.setItem('username', payload.username)
            }
          } catch (e) {
            console.error('解析 token 失败', e)
          }

          ElMessage.success('登录成功！')
          router.push('/dashboard')
        } else {
          ElMessage.error('登录失败：未获取到令牌')
        }
      } catch (error) {
        console.log('登录失败，错误已在拦截器中显示')
      } finally {
        loading.value = false
      }
    }
  })
}

const goToRegister = () => {
  router.push('/register')
}
</script>

<style scoped>
.login-container {
  height: 100vh;
  width: 100vw;
  display: flex;
  justify-content: center;
  align-items: center;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  margin: 0;
  padding: 0;
  overflow: hidden;
}

.login-box {
  background: white;
  padding: 40px;
  border-radius: 10px;
  box-shadow: 0 10px 40px rgba(0,0,0,0.2);
  width: 400px;
}

.login-form {
  padding: 0;
}

.login-form :deep(.el-form-item__content) {
  display: flex;
  justify-content: center;
}

.login-form :deep(.el-input) {
  width: 100%;
}
</style>