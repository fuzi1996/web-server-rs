# GitHub Actions 工作流说明

本项目包含以下 GitHub Actions 工作流：

## 工作流概览

### 1. CI (持续集成)
- **文件**: `.github/workflows/ci.yml`
- **触发**: 推送到 main/master 分支或创建 Pull Request
- **功能**: 
  - 代码检查 (`cargo check`)
  - 运行测试 (`cargo test`)
  - 代码质量检查 (`cargo clippy`)
  - 代码格式检查 (`cargo fmt`)
  - 安全审计 (`cargo audit`)

### 2. Build (构建)
- **文件**: `.github/workflows/build.yml`
- **触发**: 推送到 main/master 分支或创建 Pull Request
- **功能**:
  - 多平台构建 (Linux, Windows, macOS)
  - 上传构建产物

### 3. Release (发布)
- **文件**: `.github/workflows/release.yml`
- **触发**: 手动触发 (workflow_dispatch)
- **功能**:
  - 构建多平台可执行文件
  - 创建 GitHub Release
  - 上传可执行文件到 Release 页面

## 使用方法

### 手动发布 Release

1. 进入 GitHub 仓库页面
2. 点击 "Actions" 标签页
3. 选择 "Release" 工作流
4. 点击 "Run workflow" 按钮
5. 填写版本号 (例如: v1.0.0)
6. 填写发布说明 (可选)
7. 点击 "Run workflow" 开始构建

### 输入参数

- **version**: 发布版本号 (必需)
  - 格式: v1.0.0, v1.1.0 等
  - 示例: v1.0.0

- **release_notes**: 发布说明 (可选)
  - 默认值: "Release"
  - 支持 Markdown 格式

### 输出文件

Release 工作流会生成以下可执行文件：

- `web-server-linux-x64` - Linux x64 版本
- `web-server-windows-x64.exe` - Windows x64 版本
- `web-server-macos-x64` - macOS x64 版本

## 工作流配置

### 支持的平台

- **Linux**: Ubuntu 20.04 (x86_64)
- **Windows**: Windows Server 2019 (x86_64)
- **macOS**: macOS 12 (x86_64)

### Rust 版本

- **稳定版**: 最新稳定版 Rust
- **最低版本**: Rust 1.76
- **测试版本**: stable, 1.76

### 缓存策略

- 缓存 Cargo 依赖
- 缓存构建产物
- 基于 Cargo.lock 的缓存键

## 故障排除

### 常见问题

1. **构建失败**
   - 检查代码是否有语法错误
   - 确保所有依赖都正确配置
   - 查看构建日志获取详细错误信息

2. **Release 创建失败**
   - 确保版本号格式正确 (v1.0.0)
   - 检查是否有足够的权限创建 Release
   - 确保仓库设置中启用了 Actions

3. **上传失败**
   - 检查网络连接
   - 确保文件大小在限制范围内
   - 查看上传日志获取详细错误信息

### 调试步骤

1. 查看 Actions 日志
2. 检查工作流配置
3. 验证输入参数
4. 确认权限设置

## 自定义配置

### 添加新平台

在 `release.yml` 的 matrix 中添加新平台：

```yaml
- os: ubuntu-latest
  target: aarch64-unknown-linux-gnu
  asset_name: web-server-linux-arm64
```

### 修改构建参数

可以修改以下参数：

- `toolchain`: Rust 工具链版本
- `target`: 目标平台
- `asset_name`: 输出文件名

### 添加新步骤

在工作流中添加新的构建或测试步骤：

```yaml
- name: Custom step
  run: |
    echo "Custom build step"
    # 你的自定义命令
```

## 安全考虑

- 使用 `GITHUB_TOKEN` 进行身份验证
- 限制工作流权限
- 定期更新 Actions 版本
- 监控依赖安全漏洞

## 性能优化

- 使用缓存减少构建时间
- 并行构建多个平台
- 优化依赖下载
- 使用预构建的 Docker 镜像 