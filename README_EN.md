# Web Server - Static File Server

An efficient static file web server written in Rust.

## Features

- ✅ Static file serving
- ✅ Directory browsing
- ✅ Multi-threaded concurrency
- ✅ Security protection
- ✅ Configurable port and working directory

## Quick Start

### Install
```bash
# Clone the project
git clone <repository-url>
cd web-server

# Build
cargo build --release
```

### Run
```bash
# Basic usage
cargo run <port> <working-directory>

# Examples
cargo run 8080 .                    # Port 8080, current directory
cargo run 7878 /path/to/files       # Port 7878, specified directory
```

### Access
```
http://127.0.0.1:<port>/
```

## Configuration

### Environment Variables
```bash
export LOG_LEVEL=debug  # Log levels: trace, debug, info, warn, error
```

### Command Line Arguments
- **Port**: Listening port (default: 7878)
- **Working Directory**: Service directory (default: current directory)

## Project Structure
```
web-server/
├── src/
│   ├── main.rs          # Main program
│   └── lib.rs           # Thread pool implementation
├── Cargo.toml           # Project configuration
├── 404.html             # 404 error page
└── README.md            # Documentation
```

## Development

```bash
cargo build              # Build
cargo test               # Test
cargo clippy             # Code check
cargo fmt                # Format
```

## Release

### Manual Release
1. GitHub Actions → Release workflow
2. Fill in version number and description
3. Wait for build completion

### Download Pre-built Versions
Download from [Releases](https://github.com/fuzi1996/web-server-rs/releases):
- `web-server-linux-x64`
- `web-server-windows-x64.exe`
- `web-server-macos-x64`

## Troubleshooting

### Common Issues
- **Port in use**: Use a different port
- **Directory doesn't exist**: Ensure working directory exists
- **Permission denied**: Check file read permissions

### Debug Logging
```bash
LOG_LEVEL=debug cargo run 8080 .
```

## License

MIT License - see [LICENSE](LICENSE) file for details. 