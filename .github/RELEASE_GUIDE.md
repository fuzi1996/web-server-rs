# 发布指南 / Release Guide

## 中文指南

### 自动发布 (推荐)

使用 GitHub Actions 进行自动发布：

1. **准备工作**
   - 确保代码已推送到 GitHub 仓库
   - 确保 GitHub Actions 已启用

2. **创建发布**
   - 进入 GitHub 仓库页面
   - 点击 "Actions" 标签页
   - 选择 "Release" 工作流
   - 点击 "Run workflow"
   - 填写版本号 (如: v1.0.0)
   - 填写发布说明
   - 点击 "Run workflow"

3. **等待构建**
   - 构建过程大约需要 5-10 分钟
   - 可以在 Actions 页面查看构建进度
   - 构建完成后会自动创建 Release

4. **下载文件**
   - 进入 Releases 页面
   - 下载对应平台的可执行文件

### 手动发布

#### 使用脚本 (推荐)

**Linux/macOS:**
```bash
# 给脚本执行权限
chmod +x scripts/build-release.sh

# 运行构建脚本
./scripts/build-release.sh
```

**Windows:**
```powershell
# 运行 PowerShell 脚本
.\scripts\build-release.ps1
```

#### 手动构建

1. **安装目标平台**
```bash
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-pc-windows-msvc
rustup target add x86_64-apple-darwin
```

2. **构建各平台版本**
```bash
# Linux
cargo build --release --target x86_64-unknown-linux-gnu
cp target/x86_64-unknown-linux-gnu/release/web-server web-server-linux-x64

# Windows
cargo build --release --target x86_64-pc-windows-msvc
cp target/x86_64-pc-windows-msvc/release/web-server.exe web-server-windows-x64.exe

# macOS
cargo build --release --target x86_64-apple-darwin
cp target/x86_64-apple-darwin/release/web-server web-server-macos-x64
```

3. **创建 Release**
   - 在 GitHub 上创建新的 Release
   - 上传构建的可执行文件
   - 填写版本号和发布说明

### 版本号规范

使用语义化版本号：

- **主版本号**: 不兼容的 API 修改
- **次版本号**: 向下兼容的功能性新增
- **修订号**: 向下兼容的问题修正

示例：
- `v1.0.0` - 第一个正式版本
- `v1.1.0` - 添加新功能
- `v1.1.1` - 修复 bug

---

## English Guide

### Automated Release (Recommended)

Use GitHub Actions for automated releases:

1. **Preparation**
   - Ensure code is pushed to GitHub repository
   - Ensure GitHub Actions is enabled

2. **Create Release**
   - Go to GitHub repository page
   - Click "Actions" tab
   - Select "Release" workflow
   - Click "Run workflow"
   - Fill in version number (e.g., v1.0.0)
   - Fill in release notes
   - Click "Run workflow"

3. **Wait for Build**
   - Build process takes about 5-10 minutes
   - Monitor build progress in Actions page
   - Release will be created automatically when build completes

4. **Download Files**
   - Go to Releases page
   - Download executable files for your platform

### Manual Release

#### Using Scripts (Recommended)

**Linux/macOS:**
```bash
# Make script executable
chmod +x scripts/build-release.sh

# Run build script
./scripts/build-release.sh
```

**Windows:**
```powershell
# Run PowerShell script
.\scripts\build-release.ps1
```

#### Manual Build

1. **Install Targets**
```bash
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-pc-windows-msvc
rustup target add x86_64-apple-darwin
```

2. **Build for Each Platform**
```bash
# Linux
cargo build --release --target x86_64-unknown-linux-gnu
cp target/x86_64-unknown-linux-gnu/release/web-server web-server-linux-x64

# Windows
cargo build --release --target x86_64-pc-windows-msvc
cp target/x86_64-pc-windows-msvc/release/web-server.exe web-server-windows-x64.exe

# macOS
cargo build --release --target x86_64-apple-darwin
cp target/x86_64-apple-darwin/release/web-server web-server-macos-x64
```

3. **Create Release**
   - Create new Release on GitHub
   - Upload built executables
   - Fill in version number and release notes

### Version Number Convention

Use semantic versioning:

- **Major**: Incompatible API changes
- **Minor**: Backward-compatible functionality additions
- **Patch**: Backward-compatible bug fixes

Examples:
- `v1.0.0` - First stable release
- `v1.1.0` - Add new features
- `v1.1.1` - Fix bugs

## 文件说明 / File Descriptions

### 输出文件

- `web-server-linux-x64` - Linux x64 可执行文件
- `web-server-windows-x64.exe` - Windows x64 可执行文件
- `web-server-macos-x64` - macOS x64 可执行文件

### 工作流文件

- `.github/workflows/ci.yml` - 持续集成
- `.github/workflows/build.yml` - 构建工作流
- `.github/workflows/release.yml` - 发布工作流

### 脚本文件

- `scripts/build-release.sh` - Linux/macOS 构建脚本
- `scripts/build-release.ps1` - Windows 构建脚本

## 故障排除 / Troubleshooting

### 常见问题

1. **构建失败**
   - 检查 Rust 版本是否支持
   - 确保所有依赖正确安装
   - 查看构建日志获取详细错误

2. **权限问题**
   - 确保有创建 Release 的权限
   - 检查 GitHub Actions 权限设置

3. **网络问题**
   - 检查网络连接
   - 确保可以访问 GitHub

### 调试步骤

1. 查看 Actions 日志
2. 检查工作流配置
3. 验证输入参数
4. 确认权限设置

## 最佳实践 / Best Practices

1. **版本管理**
   - 使用语义化版本号
   - 保持版本号一致性
   - 及时更新 CHANGELOG

2. **发布流程**
   - 在发布前进行充分测试
   - 编写清晰的发布说明
   - 验证所有平台的可执行文件

3. **文档维护**
   - 更新 README 文件
   - 记录重要变更
   - 保持文档与代码同步

---

*最后更新 / Last updated: 2025年1月* 