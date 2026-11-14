# syntax=docker/dockerfile:1
# 后端镜像：多阶段构建，构建上下文为仓库根目录（见 docker-compose.yml）
#
# 注意：v1.0.0 的 erp-backend/src/main.rs 绑定 127.0.0.1:3000，
# 在标准 bridge 网络下其他容器无法访问。部署前请按 docs/deployment.md §5.2
# 自行把监听地址改为 0.0.0.0（或改为读取环境变量）。

FROM rust:1.98-bookworm AS builder
WORKDIR /app

RUN apt-get update \
 && apt-get install -y --no-install-recommends pkg-config libssl-dev \
 && rm -rf /var/lib/apt/lists/*

# workspace 根包 + 后端成员，缺一不可（根 Cargo.toml 含 [package] ERP）
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY erp-backend ./erp-backend

RUN cargo build --release -p erp-backend

FROM debian:bookworm-slim
RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates libssl3 \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app
RUN mkdir -p /app/logs

COPY --from=builder /app/target/release/erp-backend /usr/local/bin/erp-backend

ENV RUST_LOG=info
EXPOSE 3000

# 无 shell 依赖，直接执行二进制；日志写入 /app/logs（compose 挂载卷）
CMD ["erp-backend"]
