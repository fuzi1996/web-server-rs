# 使用示例 / Usage Examples

## 中文示例

### 基本用法

```bash
# 1. 编译项目
cargo build --release

# 2. 启动服务器（默认端口7878，当前目录）
cargo run

# 3. 指定端口启动
cargo run 8080

# 4. 指定端口和工作目录
cargo run 8080 /path/to/your/files

# 5. 设置日志级别
LOG_LEVEL=debug cargo run 8080 .
```

### 实际使用场景

#### 场景1：本地开发服务器
```bash
# 在项目目录中启动开发服务器
cd my-project
cargo run 3000 .
# 访问 http://127.0.0.1:3000/
```

#### 场景2：文档服务器
```bash
# 服务文档目录
cargo run 8080 /path/to/docs
# 访问 http://127.0.0.1:8080/
```

#### 场景3：图片服务器
```bash
# 服务图片目录
cargo run 9000 /path/to/images
# 访问 http://127.0.0.1:9000/
```

### 测试命令

```bash
# 测试根目录访问
curl http://127.0.0.1:8080/

# 测试文件访问
curl http://127.0.0.1:8080/hello.html

# 测试404页面
curl http://127.0.0.1:8080/nonexistent.html

# 测试目录访问
curl http://127.0.0.1:8080/src/
```

---

## English Examples

### Basic Usage

```bash
# 1. Build the project
cargo build --release

# 2. Start server (default port 7878, current directory)
cargo run

# 3. Start with custom port
cargo run 8080

# 4. Start with custom port and working directory
cargo run 8080 /path/to/your/files

# 5. Set log level
LOG_LEVEL=debug cargo run 8080 .
```

### Real-world Scenarios

#### Scenario 1: Local Development Server
```bash
# Start development server in project directory
cd my-project
cargo run 3000 .
# Access http://127.0.0.1:3000/
```

#### Scenario 2: Documentation Server
```bash
# Serve documentation directory
cargo run 8080 /path/to/docs
# Access http://127.0.0.1:8080/
```

#### Scenario 3: Image Server
```bash
# Serve image directory
cargo run 9000 /path/to/images
# Access http://127.0.0.1:9000/
```

### Test Commands

```bash
# Test root directory access
curl http://127.0.0.1:8080/

# Test file access
curl http://127.0.0.1:8080/hello.html

# Test 404 page
curl http://127.0.0.1:8080/nonexistent.html

# Test directory access
curl http://127.0.0.1:8080/src/
```

## 常见问题 / Common Issues

### 中文

**Q: 端口被占用怎么办？**
A: 使用其他端口或停止占用该端口的程序
```bash
# 查看端口占用
netstat -an | findstr :8080

# 使用其他端口
cargo run 8081 .
```

**Q: 工作目录不存在怎么办？**
A: 确保指定的目录存在
```bash
# 检查目录是否存在
ls /path/to/directory

# 使用当前目录
cargo run 8080 .
```

**Q: 权限不足怎么办？**
A: 确保有读取工作目录的权限
```bash
# 检查权限
ls -la /path/to/directory

# 使用有权限的目录
cargo run 8080 /home/user/public
```

### English

**Q: What if the port is already in use?**
A: Use a different port or stop the program using that port
```bash
# Check port usage
netstat -an | findstr :8080

# Use different port
cargo run 8081 .
```

**Q: What if the working directory doesn't exist?**
A: Make sure the specified directory exists
```bash
# Check if directory exists
ls /path/to/directory

# Use current directory
cargo run 8080 .
```

**Q: What if permission is denied?**
A: Ensure you have read permissions for the working directory
```bash
# Check permissions
ls -la /path/to/directory

# Use directory with proper permissions
cargo run 8080 /home/user/public
``` 