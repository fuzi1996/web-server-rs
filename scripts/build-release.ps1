# 构建发布脚本 (Windows PowerShell)
# 用于本地构建多平台可执行文件

param(
    [string]$Version = "local"
)

# 设置错误处理
$ErrorActionPreference = "Stop"

# 颜色函数
function Write-Info {
    param([string]$Message)
    Write-Host "[INFO] $Message" -ForegroundColor Green
}

function Write-Warn {
    param([string]$Message)
    Write-Host "[WARN] $Message" -ForegroundColor Yellow
}

function Write-Error {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor Red
}

# 检查 Rust 是否安装
function Test-Rust {
    try {
        $rustVersion = rustc --version
        Write-Info "Rust 版本: $rustVersion"
        return $true
    }
    catch {
        Write-Error "Rust 未安装，请先安装 Rust"
        return $false
    }
}

# 安装目标平台
function Install-Targets {
    Write-Info "安装目标平台..."
    
    $targets = @(
        "x86_64-unknown-linux-gnu",
        "x86_64-pc-windows-msvc",
        "x86_64-apple-darwin"
    )
    
    foreach ($target in $targets) {
        Write-Info "安装目标: $target"
        rustup target add $target
    }
}

# 构建指定平台
function Build-Target {
    param(
        [string]$Target,
        [string]$OutputName
    )
    
    Write-Info "构建 $Target..."
    
    try {
        cargo build --release --target $Target
        
        if ($Target -like "*windows*") {
            Copy-Item "target/$Target/release/web-server.exe" $OutputName
        } else {
            Copy-Item "target/$Target/release/web-server" $OutputName
        }
        
        Write-Info "构建成功: $OutputName"
    }
    catch {
        Write-Error "构建失败: $Target"
        throw
    }
}

# 主函数
function Main {
    Write-Info "开始构建发布版本..."
    
    # 检查 Rust
    if (-not (Test-Rust)) {
        exit 1
    }
    
    # 安装目标平台
    Install-Targets
    
    # 创建输出目录
    if (-not (Test-Path "dist")) {
        New-Item -ItemType Directory -Path "dist" | Out-Null
    }
    
    # 构建各平台版本
    Build-Target "x86_64-unknown-linux-gnu" "dist/web-server-linux-x64"
    Build-Target "x86_64-pc-windows-msvc" "dist/web-server-windows-x64.exe"
    Build-Target "x86_64-apple-darwin" "dist/web-server-macos-x64"
    
    Write-Info "所有平台构建完成！"
    Write-Info "输出文件:"
    Get-ChildItem "dist" | Format-Table Name, Length, LastWriteTime
    
    # 计算文件大小
    Write-Info "文件大小:"
    Get-ChildItem "dist" | ForEach-Object {
        $size = [math]::Round($_.Length / 1MB, 2)
        Write-Host "$($_.Name): $size MB"
    }
}

# 运行主函数
try {
    Main
}
catch {
    Write-Error "构建过程中发生错误: $($_.Exception.Message)"
    exit 1
} 