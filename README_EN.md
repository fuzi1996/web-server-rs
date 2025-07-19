# Web Server - Static File Server

A simple, efficient static file web server written in Rust.

## Features

- ✅ **Static File Serving** - Support for all types of static files
- ✅ **Directory Browsing** - Auto-generated directory index pages
- ✅ **404 Error Handling** - Custom 404 error pages
- ✅ **Thread Pool** - Multi-threaded concurrent request processing
- ✅ **Logging** - Detailed request and error logging
- ✅ **Security Protection** - Protection against directory traversal attacks
- ✅ **Configurable Port** - Support for custom ports
- ✅ **Configurable Working Directory** - Support for specifying service directories

## Quick Start

### Prerequisites

Make sure you have Rust installed:

```bash
# Check Rust version
rustc --version
cargo --version
```

### Build the Project

```bash
# Clone the project
git clone <repository-url>
cd web-server

# Build the project
cargo build --release
```

### Run the Server

```bash
# Basic usage: specify port and working directory
cargo run <port> <working-directory>

# Example: serve current directory on port 8080
cargo run 8080 .

# Example: serve specified directory on port 7878
cargo run 7878 /path/to/your/files
```

## Usage

### Command Line Arguments

```bash
cargo run <port> <working-directory>
```

- **Port**: The port number for the server to listen on (default: 7878)
- **Working Directory**: The path to the directory containing files to serve (default: current directory)

### Environment Variables

```bash
# Set log level
export LOG_LEVEL=debug  # Options: trace, debug, info, warn, error
```

### Access the Server

After starting the server, access it in your browser:

```
http://127.0.0.1:<port>/
```

## Features Explained

### Static File Serving

The server automatically detects file types and returns appropriate HTTP responses:

- **HTML files**: Returns `text/html` content type
- **CSS files**: Returns `text/css` content type
- **JavaScript files**: Returns `application/javascript` content type
- **Image files**: Returns appropriate image MIME types
- **Other files**: Returns `application/octet-stream` type

### Directory Browsing

When accessing a directory path, the server generates an HTML page containing links to all files and subdirectories.

### Security Features

- **Path Validation**: Protection against directory traversal attacks
- **Absolute Path Checking**: Ensures only files within the working directory can be accessed
- **Error Handling**: Graceful handling of file not found and other errors

### Logging

The server logs the following information:

- Server startup information
- Each HTTP request
- File access paths
- Error messages

## Project Structure

```
web-server/
├── src/
│   └── main.rs          # Main program file
├── Cargo.toml           # Project configuration and dependencies
├── Cargo.lock           # Dependency lock file
├── 404.html             # Custom 404 error page
├── hello.html           # Example HTML file
└── README.md            # Project documentation
```

## Technical Implementation

### Core Components

1. **HTTP Parser** - Parses HTTP request headers
2. **File Handler** - Handles static file requests
3. **Directory Handler** - Generates directory browsing pages
4. **Thread Pool** - Processes multiple requests concurrently
5. **Logging System** - Records server activities

### Key Features

- **Non-blocking I/O**: Uses Rust's asynchronous I/O features
- **Memory Safety**: Leverages Rust's ownership system for memory safety
- **Error Handling**: Comprehensive error handling mechanisms
- **Performance Optimization**: Efficient thread pool implementation

## Development

### Build Development Version

```bash
cargo build
```

### Run Tests

```bash
cargo test
```

### Code Checking

```bash
cargo check
cargo clippy
```

## Examples

### Starting the Server

```bash
# Serve current directory on port 8080
cargo run 8080 .

# Output example:
# [INFO] Server is running on port 8080 at .
```

### Accessing Files

```bash
# Access root directory
curl http://127.0.0.1:8080/

# Access specific file
curl http://127.0.0.1:8080/hello.html

# Access non-existent file (returns 404)
curl http://127.0.0.1:8080/nonexistent.html
```

## Troubleshooting

### Common Issues

1. **Port Already in Use**
   ```
   [ERROR] Failed to bind to port 8080: Address already in use
   ```
   Solution: Use a different port or stop the program using that port

2. **Working Directory Does Not Exist**
   ```
   [ERROR] Work directory /path/to/dir does not exist
   ```
   Solution: Ensure the specified working directory exists

3. **Permission Denied**
   ```
   [ERROR] Permission denied
   ```
   Solution: Ensure you have read permissions for the working directory

### Log Levels

Set different log levels to get more detailed information:

```bash
# Detailed logging
LOG_LEVEL=debug cargo run 8080 .

# Only show errors
LOG_LEVEL=error cargo run 8080 .
```

## API Reference

### HTTP Endpoints

- `GET /` - Returns directory listing for root directory
- `GET /<path>` - Returns file content or directory listing
- `GET /404.html` - Custom 404 error page

### Response Format

#### Directory Listing
```html
<a href="./file1.txt">./file1.txt</a><br/>
<a href="./subdir/">./subdir/</a><br/>
```

#### File Response
```
HTTP/1.1 200 OK
Content-Length: <file-size>

<file-content>
```

#### Error Response
```
HTTP/1.1 404 NOT FOUND
Content-Length: <error-page-size>

<error-page-content>
```

## Performance

### Benchmarks

- **Concurrent Connections**: Supports multiple simultaneous connections via thread pool
- **File Serving**: Efficient file reading and serving
- **Memory Usage**: Low memory footprint due to Rust's memory management

### Optimization Tips

1. Use `cargo build --release` for production builds
2. Set appropriate log levels to reduce overhead
3. Consider using a reverse proxy for high-traffic scenarios

## Security Considerations

### Implemented Security Measures

- **Path Traversal Protection**: Prevents access to files outside working directory
- **Input Validation**: Validates all user inputs
- **Error Information**: Limits sensitive information in error messages

### Best Practices

1. Run the server with minimal required permissions
2. Regularly update dependencies
3. Monitor logs for suspicious activity
4. Use HTTPS in production environments

## Contributing

We welcome contributions! Please feel free to submit issues and pull requests.

### Development Setup

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

### Code Style

- Follow Rust coding conventions
- Use meaningful variable and function names
- Add comments for complex logic
- Ensure all code compiles without warnings

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Changelog

### v0.1.0
- Initial release
- Static file serving support
- Directory browsing functionality
- Thread pool implementation
- Logging system integration

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Uses [log](https://crates.io/crates/log) for logging
- Uses [env_logger](https://crates.io/crates/env_logger) for log initialization

## Support

If you encounter any issues or have questions:

1. Check the [troubleshooting](#troubleshooting) section
2. Search existing [issues](../../issues)
3. Create a new issue with detailed information

## Roadmap

- [ ] HTTPS support
- [ ] Gzip compression
- [ ] Cache headers
- [ ] Configuration file support
- [ ] Metrics and monitoring
- [ ] Docker support 