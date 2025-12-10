# 故障排查指南 (Troubleshooting Guide)

## 运行时错误诊断 (Runtime Error Diagnosis)

如果应用程序无法加载数据，请按以下步骤排查：

### 1. 启用详细日志 (Enable Verbose Logging)

应用程序现在包含调试日志输出。运行时会在控制台显示：
- API 请求的 URL
- 加载的应用数量
- 任何错误信息

```bash
# 运行应用并查看日志
cargo run

# 或使用 RUST_BACKTRACE 获取更详细的错误信息
RUST_BACKTRACE=1 cargo run
```

### 2. 常见问题 (Common Issues)

#### 问题: "Failed to fetch app list" 或网络错误
**原因**: 
- 网络连接问题
- 防火墙阻止
- DNS 解析失败

**解决方案**:
```bash
# 测试网络连接
curl -v https://cdn-d.spark-app.store/store/all/applist.json

# 检查 DNS
nslookup cdn-d.spark-app.store
nslookup search.deepinos.org.cn
```

#### 问题: "Failed to parse JSON"
**原因**: 
- 服务器返回的数据格式不正确
- API 响应格式已改变

**解决方案**:
```bash
# 查看服务器返回的实际数据
curl https://cdn-d.spark-app.store/store/all/applist.json | head -100
```

#### 问题: Thread panic 或 runtime error
**原因**: 
- Tokio 运行时冲突（已在 commit eef90b1 修复）
- Async 运行时配置问题

**解决方案**:
- 更新到最新代码
- 确保使用正确的 reqwest 配置（rustls-tls）

### 3. 测试 API 连接 (Test API Connection)

创建测试文件验证 API：

```bash
# 测试应用列表 API
curl https://cdn-d.spark-app.store/store/all/applist.json

# 测试搜索 API
curl "https://search.deepinos.org.cn/appinfo/search?keyword=firefox"
```

### 4. 架构检测 (Architecture Detection)

应用会自动检测系统架构：
- x86_64 → 使用 `store/` 端点
- aarch64 → 使用 `aarch64-store/` 端点  
- loongarch64 → 使用 `loong64-store/` 端点

检查您的架构：
```bash
uname -m
```

### 5. 报告问题 (Report Issues)

如果问题仍然存在，请提供：
1. 完整的错误日志（包括 thread panic 信息）
2. 操作系统和架构 (`uname -a`)
3. Rust 版本 (`rustc --version`)
4. 网络测试结果（curl 命令输出）

## 依赖说明 (Dependencies)

当前配置避免了运行时冲突：
- ✅ reqwest 使用 rustls-tls（不依赖 tokio）
- ✅ 移除了显式的 tokio 依赖
- ✅ iced 使用自己的 async 运行时
- ✅ 所有组件兼容工作

## 性能优化建议 (Performance Tips)

1. 首次运行可能较慢（需要下载和解析 JSON）
2. 搜索至少需要 2 个字符才会触发
3. 每次最多显示 50 个应用
4. 切换分类会触发新的 API 请求
