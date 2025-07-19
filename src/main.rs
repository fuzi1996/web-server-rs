use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let connection = stream.unwrap();
        hello_connection(connection);
    }
}

const OK_STATUS_LINE: &str = "HTTP/1.1 200 OK";
const NOT_FOUND_STATUS_LINE: &str = "HTTP/1.1 404 NOT FOUND";
fn hello_connection(mut stream: TcpStream) {
    let buffer = BufReader::new(&stream);
    let http_request = buffer.lines().next().unwrap().unwrap();

    let request_resource = http_request.split_whitespace().nth(1);

    if let Some(resource) = request_resource {
        if resource == "/" {
            deal_root_resource(resource);
            return;
        } else {
            deal_other_resource(resource);
            return;
        }
    }

    // 读取404.html文件内容
    let not_found_content = match fs::read_to_string("404.html") {
        Ok(content) => content,
        Err(_) => {
            String::from("<h1>404 Not Found</h1><p>The requested resource was not found.</p>")
        }
    };

    let response = format!(
        "{}\r\nContent-Length:{}\r\n\r\n{}",
        NOT_FOUND_STATUS_LINE,
        not_found_content.len(),
        not_found_content
    );
    stream.write_all(response.as_bytes()).unwrap();
}

fn deal_other_resource(resource: &str) -> Vec<u8> {
    let file_path = format!("./{}", resource);
    println!("file_path: {}", file_path);
    let file_content = fs::read_to_string(file_path).unwrap();
    with_content_response(file_content)
}

fn deal_root_resource(_resource: &str) -> Vec<u8> {
    let read_dir = fs::read_dir(".").unwrap();

    let mut resources: Vec<String> = Vec::new();
    for dir in read_dir {
        let dir_entry = dir.unwrap();
        let path = dir_entry.path();
        let path = path.to_str().unwrap();

        let link_str = format!("<a href=\"{}\">{}</a><br/>", path, path);
        resources.push(link_str);
    }
    let resource_content = resources.join("");

    with_content_response(resource_content)
}

fn with_content_response(response: String) -> Vec<u8> {
    let response = format!(
        "{}\r\nContent-Length:{}\r\n\r\n{}",
        OK_STATUS_LINE,
        response.len(),
        response
    );
    response.into_bytes()
}
