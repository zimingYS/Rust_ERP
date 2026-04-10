import axios from 'axios'
import { ElMessage } from 'element-plus'
import router from '../router'

// 创建 axios 实例
const request = axios.create({
    baseURL: 'http://127.0.0.1:3000',
    timeout: 10000,
})

// 请求拦截器
request.interceptors.request.use(
    config => {
        config.headers['Content-Type'] = 'application/json'
        const token = localStorage.getItem('erp_token')
        if (token) {
            config.headers.Authorization = `Bearer ${token}`
        }
        return config
    },
    error => {
        return Promise.reject(error)
    }
)

// 响应拦截器
request.interceptors.response.use(
    response => {
        return response.data
    },
    error => {
        if (error.response) {
            const errorMsg = error.response.data?.error || '请求失败'

            switch (error.response.status) {
                case 401:
                    // 只有在已登录状态下才退出
                    if (localStorage.getItem('erp_token')) {
                        ElMessage.error('登录失效，请重新登录')
                        localStorage.removeItem('erp_token')
                        router.push('/login')
                    } else {
                        // 登录时的 401 错误，显示后端返回的具体错误
                        ElMessage.error(errorMsg)
                    }
                    break
                case 400:
                    ElMessage.error(errorMsg)
                    break
                case 500:
                    ElMessage.error('服务器内部错误')
                    break
                default:
                    ElMessage.error(errorMsg)
            }
        } else {
            ElMessage.error('网络连接失败，请检查后端服务是否启动')
        }
        return Promise.reject(error)
    }
)

export default request