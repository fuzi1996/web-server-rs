# Web Server - 静态文件服务器

一个用 Rust 编写的高效静态文件 Web 服务器。

## 功能特性

- ✅ 静态文件服务
- ✅ 目录浏览
- ✅ 多线程并发
- ✅ 安全防护
- ✅ 可配置端口和工作目录

## 快速开始

### 安装
```bash
# 克隆项目
git clone <repository-url>
cd web-server

# 编译
cargo build --release
```

### 运行
```bash
# 基本用法
cargo run <端口> <工作目录>

# 示例
cargo run 8080 .                    # 8080端口，当前目录
cargo run 7878 /path/to/files       # 7878端口，指定目录
```

### 访问
```
http://127.0.0.1:<端口>/
```

## 配置

### 环境变量
```bash
export LOG_LEVEL=debug  # 日志级别: trace, debug, info, warn, error
```

### 命令行参数
- **端口**: 监听端口 (默认: 7878)
- **工作目录**: 服务目录 (默认: 当前目录)

## 项目结构
```
web-server/
├── src/
│   ├── main.rs          # 主程序
│   └── lib.rs           # 线程池实现
├── Cargo.toml           # 项目配置
├── 404.html             # 404错误页面
└── README.md            # 说明文档
```

## 开发

```bash
cargo build              # 构建
cargo test               # 测试
cargo clippy             # 代码检查
cargo fmt                # 格式化
```

## 发布

### 手动发布
1. GitHub Actions → Release 工作流
2. 填写版本号和说明
3. 等待构建完成

### 下载预构建版本
在 [Releases](https://github.com/fuzi1996/web-server-rs/releases) 页面下载：
- `web-server-linux-x64`
- `web-server-windows-x64.exe`
- `web-server-macos-x64`

## 故障排除

### 常见问题
- **端口被占用**: 使用其他端口
- **目录不存在**: 确保工作目录存在
- **权限不足**: 检查文件读取权限

### 日志调试
```bash
LOG_LEVEL=debug cargo run 8080 .
```

## 许可证

MIT License - 详见 [LICENSE](LICENSE) 文件。 