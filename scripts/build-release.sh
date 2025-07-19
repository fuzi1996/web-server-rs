#!/bin/bash

# 构建发布脚本
# 用于本地构建多平台可执行文件

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 打印带颜色的消息
print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 检查 Rust 是否安装
check_rust() {
    if ! command -v rustc &> /dev/null; then
        print_error "Rust 未安装，请先安装 Rust"
        exit 1
    fi
    print_info "Rust 版本: $(rustc --version)"
}

# 安装目标平台
install_targets() {
    print_info "安装目标平台..."
    rustup target add x86_64-unknown-linux-gnu
    rustup target add x86_64-pc-windows-msvc
    rustup target add x86_64-apple-darwin
}

# 构建指定平台
build_target() {
    local target=$1
    local output_name=$2
    
    print_info "构建 $target..."
    
    if cargo build --release --target $target; then
        if [[ "$target" == *"windows"* ]]; then
            cp target/$target/release/web-server.exe $output_name
        else
            cp target/$target/release/web-server $output_name
        fi
        print_info "构建成功: $output_name"
    else
        print_error "构建失败: $target"
        exit 1
    fi
}

# 主函数
main() {
    print_info "开始构建发布版本..."
    
    # 检查 Rust
    check_rust
    
    # 安装目标平台
    install_targets
    
    # 创建输出目录
    mkdir -p dist
    
    # 构建各平台版本
    build_target "x86_64-unknown-linux-gnu" "dist/web-server-linux-x64"
    build_target "x86_64-pc-windows-msvc" "dist/web-server-windows-x64.exe"
    build_target "x86_64-apple-darwin" "dist/web-server-macos-x64"
    
    print_info "所有平台构建完成！"
    print_info "输出文件:"
    ls -la dist/
    
    # 计算文件大小
    print_info "文件大小:"
    if command -v du &> /dev/null; then
        du -h dist/*
    else
        ls -lh dist/
    fi
}

# 运行主函数
main "$@" 