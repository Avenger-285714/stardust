# 故障排查指南 (Troubleshooting Guide)

## 运行时错误诊断 (Runtime Error Diagnosis)

如果应用程序无法加载数据，请按以下步骤排查：

### 1. 启用详细日志 (Enable Verbose Logging)

应用程序现在包含调试日志输出。运行时会在控制台显示：
- 每个镜像服务器的尝试状态（✓ 成功 或 ✗ 失败）
- API 请求的 URL
- 加载的应用数量
- 任何错误信息

```bash
# 运行应用并查看日志
cargo run

# 或使用 RUST_BACKTRACE 获取更详细的错误信息
RUST_BACKTRACE=1 cargo run
```

应用会显示类似以下的日志：
```
[Attempt 1/3] Fetching app list from: https://mirrors.sdu.edu.cn/...
  ✓ Successfully loaded 150 apps from category: all
```

或者如果某个镜像失败：
```
[Attempt 1/3] Fetching app list from: https://mirrors.sdu.edu.cn/...
  ✗ DNS resolution failed
[Attempt 2/3] Fetching app list from: https://mirrors.sdu.edu.cn/spark-store/...
  ✓ Successfully loaded 150 apps from category: all
```

### 2. 常见问题 (Common Issues)

#### 问题: "Failed to fetch app list after trying X mirror(s)"
**原因**: 
- 网络连接问题
- 防火墙阻止所有镜像服务器
- DNS 解析失败
- 所有镜像服务器暂时不可用

**解决方案**:

应用会自动尝试多个镜像服务器。查看日志确认哪些服务器失败：

```bash
# 测试主镜像（山东大学）
curl -v https://mirrors.sdu.edu.cn/spark-store-repository/amd64-store/all/applist.json

# 测试备用镜像
curl -v https://mirrors.sdu.edu.cn/spark-store/amd64-store/all/applist.json

# 测试 Gitee 镜像
curl -v https://gitee.com/spark-store-project/spark-store/raw/master/amd64-store/all/applist.json

# 检查 DNS
nslookup mirrors.sdu.edu.cn
nslookup gitee.com
```

如果所有镜像都无法访问：
1. 检查防火墙设置
2. 尝试使用 VPN 或代理
3. 检查是否有网络限制

**注意**: 旧服务器 `cdn-d.spark-app.store` 已不再可用。当前使用山东大学镜像服务器。

#### 问题: "Failed to parse JSON"
**原因**: 
- 服务器返回的数据格式不正确
- API 响应格式已改变

**解决方案**:
```bash
# 查看服务器返回的实际数据
curl https://mirrors.sdu.edu.cn/spark-store-repository/amd64-store/all/applist.json | head -100
```

#### 问题: "Request timed out"
**原因**: 
- 网络速度慢
- 服务器响应慢

**解决方案**:
- 应用已设置 30 秒超时
- 检查网络连接速度
- 如果经常超时，可能需要更换网络或使用代理

### 3. 测试 API 连接 (Test API Connection)

创建测试文件验证 API：

```bash
# 测试应用列表 API（新镜像服务器）
curl https://mirrors.sdu.edu.cn/spark-store-repository/amd64-store/all/applist.json

# 测试其他分类
curl https://mirrors.sdu.edu.cn/spark-store-repository/amd64-store/games/applist.json
```

### 4. 架构检测 (Architecture Detection)

应用会自动检测系统架构并使用对应的端点：
- x86_64 → 使用 `amd64-store/` 端点
- aarch64 → 使用 `arm64-store/` 端点  
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
5. 是否可以访问 https://mirrors.sdu.edu.cn/

## 依赖说明 (Dependencies)

当前配置避免了运行时冲突：
- ✅ reqwest 使用 rustls-tls（不依赖 tokio）
- ✅ 移除了显式的 tokio 依赖
- ✅ iced 使用自己的 async 运行时
- ✅ 所有组件兼容工作
- ✅ 30 秒超时防止挂起

## 性能优化建议 (Performance Tips)

1. 首次运行可能较慢（需要下载和解析 JSON）
2. 搜索至少需要 2 个字符才会触发
3. 搜索现在使用本地过滤（更可靠）
4. 每次最多显示 50 个应用
5. 切换分类会触发新的 API 请求

## 更新历史 (Change History)

### 2024-12-10 (最新): 多镜像自动切换
- **添加多镜像支持**：应用现在会自动尝试多个服务器
  1. `mirrors.sdu.edu.cn/spark-store-repository/` (主镜像)
  2. `mirrors.sdu.edu.cn/spark-store/` (备用镜像)
  3. `gitee.com/spark-store-project/spark-store/raw/master/` (Gitee 镜像)
- **自动故障切换**：如果一个镜像失败，自动尝试下一个
- **详细日志**：显示每个镜像的尝试状态（✓ 成功 / ✗ 失败）
- **提高可靠性**：即使某个镜像不可用，仍可从其他镜像获取数据

### 2024-12-10: 服务器迁移
- 从 `cdn-d.spark-app.store` 迁移到 `mirrors.sdu.edu.cn/spark-store-repository/`
- 架构目录更新：`store` → `amd64-store`，`aarch64-store` → `arm64-store`
- 搜索改为本地过滤（更可靠）
- 添加 30 秒超时和详细错误消息
