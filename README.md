# Web Server - 静态文件服务器

一个用 Rust 编写的简单、高效的静态文件 Web 服务器。

## 功能特性

- ✅ **静态文件服务** - 支持所有类型的静态文件
- ✅ **目录浏览** - 自动生成目录索引页面
- ✅ **404 错误处理** - 自定义404错误页面
- ✅ **线程池** - 多线程并发处理请求
- ✅ **日志记录** - 详细的请求和错误日志
- ✅ **安全防护** - 防止目录遍历攻击
- ✅ **可配置端口** - 支持自定义端口
- ✅ **可配置工作目录** - 支持指定服务目录

## 快速开始

### 安装依赖

确保您已安装 Rust 环境：

```bash
# 检查 Rust 版本
rustc --version
cargo --version
```

### 编译项目

```bash
# 克隆项目
git clone <repository-url>
cd web-server

# 编译项目
cargo build --release
```

### 运行服务器

```bash
# 基本用法：指定端口和工作目录
cargo run <端口> <工作目录>

# 示例：在8080端口服务当前目录
cargo run 8080 .

# 示例：在7878端口服务指定目录
cargo run 7878 /path/to/your/files
```

## 使用方法

### 命令行参数

```bash
cargo run <端口> <工作目录>
```

- **端口**: 服务器监听的端口号 (默认: 7878)
- **工作目录**: 要服务的文件目录路径 (默认: 当前目录)

### 环境变量

```bash
# 设置日志级别
export LOG_LEVEL=debug  # 可选: trace, debug, info, warn, error
```

### 访问服务器

启动服务器后，在浏览器中访问：

```
http://127.0.0.1:<端口>/
```

## 功能说明

### 静态文件服务

服务器会自动检测文件类型并返回相应的 HTTP 响应：

- **HTML 文件**: 返回 `text/html` 内容类型
- **CSS 文件**: 返回 `text/css` 内容类型
- **JavaScript 文件**: 返回 `application/javascript` 内容类型
- **图片文件**: 返回相应的图片 MIME 类型
- **其他文件**: 返回 `application/octet-stream` 类型

### 目录浏览

当访问目录路径时，服务器会生成一个包含所有文件和子目录链接的 HTML 页面。

### 安全特性

- **路径验证**: 防止目录遍历攻击
- **绝对路径检查**: 确保只能访问工作目录内的文件
- **错误处理**: 优雅处理文件不存在等错误

### 日志记录

服务器会记录以下信息：

- 服务器启动信息
- 每个 HTTP 请求
- 文件访问路径
- 错误信息

## 项目结构

```
web-server/
├── src/
│   └── main.rs          # 主程序文件
├── Cargo.toml           # 项目配置和依赖
├── Cargo.lock           # 依赖锁定文件
├── 404.html             # 自定义404错误页面
├── hello.html           # 示例HTML文件
└── README.md            # 项目说明文档
```

## 技术实现

### 核心组件

1. **HTTP 解析器** - 解析 HTTP 请求头
2. **文件处理器** - 处理静态文件请求
3. **目录处理器** - 生成目录浏览页面
4. **线程池** - 并发处理多个请求
5. **日志系统** - 记录服务器活动

### 关键特性

- **非阻塞 I/O**: 使用 Rust 的异步 I/O 特性
- **内存安全**: 利用 Rust 的所有权系统确保内存安全
- **错误处理**: 完善的错误处理机制
- **性能优化**: 高效的线程池实现

## 开发

### 构建开发版本

```bash
cargo build
```

### 运行测试

```bash
cargo test
```

### 代码检查

```bash
cargo check
cargo clippy
```

## 示例

### 启动服务器

```bash
# 在8080端口服务当前目录
cargo run 8080 .

# 输出示例：
# [INFO] Server is running on port 8080 at .
```

### 访问文件

```bash
# 访问根目录
curl http://127.0.0.1:8080/

# 访问特定文件
curl http://127.0.0.1:8080/hello.html

# 访问不存在的文件（返回404）
curl http://127.0.0.1:8080/nonexistent.html
```

## 故障排除

### 常见问题

1. **端口被占用**
   ```
   [ERROR] Failed to bind to port 8080: 通常每个套接字地址只允许使用一次
   ```
   解决方案：使用其他端口或停止占用该端口的程序

2. **工作目录不存在**
   ```
   [ERROR] Work directory /path/to/dir does not exist
   ```
   解决方案：确保指定的工作目录存在

3. **权限不足**
   ```
   [ERROR] Permission denied
   ```
   解决方案：确保有读取工作目录的权限

### 日志级别

设置不同的日志级别来获取更详细的信息：

```bash
# 详细日志
LOG_LEVEL=debug cargo run 8080 .

# 只显示错误
LOG_LEVEL=error cargo run 8080 .
```

## 贡献

欢迎提交 Issue 和 Pull Request！

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

## 更新日志

### v0.1.0
- 初始版本发布
- 支持静态文件服务
- 支持目录浏览
- 实现线程池
- 添加日志记录功能 