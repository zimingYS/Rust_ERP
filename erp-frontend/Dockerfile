# syntax=docker/dockerfile:1
# 前端镜像：多阶段构建，构建上下文为仓库根目录（见 docker-compose.yml）
#
# 注意：v1.0.0 的 src/utils/request.js 把 baseURL 写死为 http://127.0.0.1:3000。
# 容器化前请按 docs/deployment.md §5.2 改为相对路径 /api 或读取 VITE_API_BASE_URL，
# 否则浏览器会请求访问者自己的 127.0.0.1。

FROM node:22-alpine AS builder
WORKDIR /app

COPY erp-frontend/package.json erp-frontend/package-lock.json ./
RUN npm ci

COPY erp-frontend/ ./
RUN npm run build

FROM nginx:1.29-alpine
COPY erp-frontend/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /app/dist /usr/share/nginx/html

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
