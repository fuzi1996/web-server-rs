use log::{error, info, LevelFilter};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::{env, fs};
use web_server::ThreadPool;

fn main() {
    // 初始化日志
    // 如果没有配置日志级别，设置默认级别为info
    if let Ok(log_level) = std::env::var("LOG_LEVEL") {
        let log_level = log_level
            .parse::<LevelFilter>()
            .unwrap_or(LevelFilter::Info);
        env_logger::Builder::from_default_env()
            .filter_level(log_level)
            .target(env_logger::Target::Stdout)
            .init();
    } else {
        env_logger::Builder::from_default_env()
            .filter_level(LevelFilter::Info)
            .target(env_logger::Target::Stdout)
            .init();
    }

    let args: Vec<String> = env::args().collect();
    let port = args
        .get(1)
        .map(|s| s.parse::<u16>().unwrap_or(7878))
        .unwrap_or(7878);
    let listener = match TcpListener::bind(format!("127.0.0.1:{port}")) {
        Ok(listener) => listener,
        Err(e) => {
            error!("Failed to bind to port {port}: {e}");
            std::process::exit(1);
        }
    };

    // 支持设置工作目录
    let work_dir = args
        .get(2)
        .map(|s| s.to_string())
        .unwrap_or(".".to_string());
    if !Path::new(&work_dir).exists() {
        error!("Work directory {work_dir} does not exist");
        std::process::exit(1);
    }
    std::env::set_current_dir(work_dir.clone()).unwrap();

    info!("Server is running on port {port} at {work_dir}");
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let connection = stream.unwrap();
        pool.execute(move || {
            hello_connection(connection);
        });
    }
}

const OK_STATUS_LINE: &str = "HTTP/1.1 200 OK";
const NOT_FOUND_STATUS_LINE: &str = "HTTP/1.1 404 NOT FOUND";
fn hello_connection(mut stream: TcpStream) {
    let buffer = BufReader::new(&stream);
    let http_request = buffer.lines().next().unwrap().unwrap();
    info!("request: {http_request}");
    let request_resource = http_request.split_whitespace().nth(1);

    if let Some(resource) = request_resource {
        let response = if resource == "/" {
            deal_root_resource(".")
        } else {
            deal_other_resource(resource)
        };
        stream.write_all(&response).unwrap();
    } else {
        stream.write_all(NOT_FOUND_STATUS_LINE.as_bytes()).unwrap();
    }
}

fn deal_not_found_resource() -> Vec<u8> {
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
    response.into_bytes()
}

fn deal_other_resource(resource: &str) -> Vec<u8> {
    let current_path = format!(".{resource}");
    // 如果文件或目录不存在，返回404
    if !Path::new(&current_path).exists() {
        return deal_not_found_resource();
    }
    // 获取绝对路径
    let file_path = Path::new(&current_path).canonicalize().unwrap();
    info!("{current_path} -> {}", file_path.display());
    // 当前路径必须在程序运行目录下
    let current_dir = std::env::current_dir().unwrap().canonicalize().unwrap();
    if !file_path.starts_with(current_dir) {
        return deal_not_found_resource();
    }

    // 判断是不是目录
    if file_path.is_dir() {
        return deal_root_resource(&current_path);
    }
    match fs::read_to_string(&file_path) {
        Ok(file_content) => {
            let response = format!(
                "{}\r\nContent-Length:{}\r\n\r\n{}",
                OK_STATUS_LINE,
                file_content.len(),
                file_content
            );
            response.into_bytes()
        }
        Err(_) => deal_not_found_resource(),
    }
}

fn deal_root_resource(_resource: &str) -> Vec<u8> {
    let read_dir = fs::read_dir(_resource).unwrap();

    let mut resources: Vec<String> = Vec::new();
    for dir in read_dir {
        let dir_entry = dir.unwrap();
        let path = dir_entry.path();
        let path = path.to_str().unwrap();

        let link_str = format!("<a href=\"{path}\">{path}</a><br/>");
        resources.push(link_str);
    }
    let resource_content = resources.join("");

    let response = format!(
        "{}\r\nContent-Length:{}\r\n\r\n{}",
        OK_STATUS_LINE,
        resource_content.len(),
        resource_content
    );
    response.into_bytes()
}
