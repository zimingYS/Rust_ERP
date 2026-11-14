# erp-frontend

Rust_ERP 的前端子项目：Vue 3 + Vite 7 + Element Plus + ECharts。

项目级说明、架构、许可与贡献规范请见 [仓库根目录 README](../README.md)。

## 开发

前置条件：Node.js ≥ 20.19、npm。

```bash
npm ci
npm run dev      # http://localhost:5173
npm run build    # 产物输出到 dist/
npm run preview  # 本地预览生产构建
```

后端需同时运行在 `http://127.0.0.1:3000`。Axios 实例在 `src/utils/request.js` 中已配置该 `baseURL`，接口路径常量集中在 `src/api/constants.js`。

> `npm ci` 依赖 `package-lock.json`，升级依赖后请一并提交锁文件，否则 CI 会失败。
