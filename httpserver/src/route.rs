use crate::handler::{Handler, StaticResourceHandler};
use http::httprequest::HttpRequest;
use http::httpresponse::HttpResponse;
use log::error;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;

pub struct Route {}

impl Route {
    pub fn route(mut connection: TcpStream) {;
        let mut buffer = BufReader::new(&connection);
        
        // 读取完整的HTTP请求
        let request_string = match Self::read_full_request(&mut buffer) {
            Ok(content) => content,
            Err(e) => {
                error!("Error reading request: {}", e);
                return;
            }
        };
        let request: HttpRequest = HttpRequest::from(request_string);

        let path = request.resource_path();
        let mut response = HttpResponse::new("404", None, None);
        if path == "/" {
            response = StaticResourceHandler::handle_request(request);
        } else if path.ends_with(".html") {
            response = StaticResourceHandler::handle_request(request);
        }

        if let Err(e) = response.send_response(&mut connection) {
            error!("Error sending response: {}", e);
        }
    }

    /// 读取完整的HTTP请求内容
    fn read_full_request(buffer: &mut BufReader<&TcpStream>) -> Result<String, std::io::Error> {
        let mut request_lines = Vec::new();
        let mut body = String::new();
        let mut content_length = 0;
        let mut transfer_encoding = String::new();
        // 读取请求行和请求头
        for line in buffer.lines() {
            let line = line?;
            if line.is_empty() {
                break; // 空行表示请求头结束
            }
            request_lines.push(line.clone());
            
            // 解析请求头
            if line.contains(":") {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() == 2 {
                    let key = parts[0].trim().to_lowercase();
                    let value = parts[1].trim();

                    if key == "content-length" {
                        content_length = value.parse::<usize>().unwrap_or(0);
                    } else if key == "transfer-encoding" {
                        transfer_encoding = value.to_string();
                    }
                }
            }
        }
        
        // 读取请求体（如果存在）
        if content_length > 0 {
            let mut body_buffer = vec![0u8; content_length];
            buffer.read_exact(&mut body_buffer)?;
            body = String::from_utf8_lossy(&body_buffer).to_string();
        } else if transfer_encoding == "chunked" {
            // 处理 chunked 编码
            if transfer_encoding == "chunked" {
                body = Self::read_chunked_body(buffer)?;
            }
        }
        
        // 组合完整的请求内容
        let mut full_request = request_lines.join("\r\n");
        full_request.push_str("\r\n\r\n");
        if !body.is_empty() {
            full_request.push_str(&body);
        }
        
        Ok(full_request)
    }
    
    /// 读取 chunked 编码的请求体
    fn read_chunked_body(buffer: &mut BufReader<&TcpStream>) -> Result<String, std::io::Error> {
        let mut body = String::new();
        
        loop {
            let mut size_line = String::new();
            buffer.read_line(&mut size_line)?;
            
            // 解析块大小
            let size_str = size_line.trim_end_matches("\r\n");
            let chunk_size = usize::from_str_radix(size_str, 16).unwrap_or(0);
            
            if chunk_size == 0 {
                // 读取最后的空行
                let mut empty_line = String::new();
                buffer.read_line(&mut empty_line)?;
                break;
            }
            
            // 读取块数据
            let mut chunk_data = vec![0u8; chunk_size];
            buffer.read_exact(&mut chunk_data)?;
            body.push_str(&String::from_utf8_lossy(&chunk_data));
            
            // 读取块结束的 \r\n
            let mut end_line = String::new();
            buffer.read_line(&mut end_line)?;
        }
        
        Ok(body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::net::{TcpListener, TcpStream as TestTcpStream};

    #[test]
    fn test_read_request_with_body() {
        // 创建一个简单的测试服务器
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        
        // 在另一个线程中发送测试请求
        let handle = std::thread::spawn(move || {
            let mut stream = TestTcpStream::connect(addr).unwrap();
            let request = "POST /test HTTP/1.1\r\nContent-Length: 11\r\n\r\nHello World";
            stream.write_all(request.as_bytes()).unwrap();
            stream.flush().unwrap();
        });
        
        // 接受连接并读取请求
        if let Ok((stream, _)) = listener.accept() {
            let mut buffer = BufReader::new(&stream);
            let result = Route::read_full_request(&mut buffer);
            
            assert!(result.is_ok());
            let request_content = result.unwrap();
            assert!(request_content.contains("POST /test HTTP/1.1"));
            assert!(request_content.contains("Hello World"));
        }
        
        handle.join().unwrap();
    }

    #[test]
    fn test_read_request_without_body() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        
        let handle = std::thread::spawn(move || {
            let mut stream = TestTcpStream::connect(addr).unwrap();
            let request = "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
            stream.write_all(request.as_bytes()).unwrap();
            stream.flush().unwrap();
        });
        
        if let Ok((stream, _)) = listener.accept() {
            let mut buffer = BufReader::new(&stream);
            let result = Route::read_full_request(&mut buffer);
            
            assert!(result.is_ok());
            let request_content = result.unwrap();
            assert!(request_content.contains("GET / HTTP/1.1"));
            assert!(!request_content.contains("Hello World"));
        }
        
        handle.join().unwrap();
    }
}
