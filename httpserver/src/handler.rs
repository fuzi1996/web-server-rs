use std::collections::HashMap;
use std::{env, fs};
use std::path::Path;

use http::httprequest::HttpRequest;
use http::httpresponse::HttpResponse;
use log::info;

pub trait Handler {
    fn handle_request(request: HttpRequest) -> HttpResponse<'static>;

    fn load_build_in_file(file_path: &str) -> Option<String> {
        // 获取工作空间根目录（httpserver的上级目录）
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let workspace_root = Path::new(manifest_dir).parent().unwrap();
        let default_path = format!("{}/public", workspace_root.display());
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let file_path = format!("{}/{}", public_path, file_path);
        if Path::new(&file_path).exists() {
            return Some(fs::read_to_string(file_path.as_str()).unwrap());
        }
        None
    }
}

pub struct StaticResourceHandler {}

impl Handler for StaticResourceHandler {
    fn handle_request(request: HttpRequest) -> HttpResponse<'static> {
        let path = request.resource_path();
        let work_dir = std::env::current_dir().unwrap();

        let current_path = format!(".{path}");
        if !Path::new(&current_path).exists() {
            return NotFoundHandler::handle_request(request);
        }

        // 获取绝对路径
        let file_path = Path::new(&current_path).canonicalize().unwrap();
        info!("{current_path} -> {}", file_path.display());
        // 当前路径必须在程序运行目录下
        let current_dir = work_dir.canonicalize().unwrap();
        if !file_path.starts_with(current_dir) {
            return NotFoundHandler::handle_request(request);
        }

        if file_path.is_dir() {
            return deal_dir_resource(file_path.to_str().unwrap(),path);
        }

        return deal_file_resource(file_path.to_str().unwrap());
    }
}

pub struct NotFoundHandler {}

impl Handler for NotFoundHandler {
    fn handle_request(_: HttpRequest) -> HttpResponse<'static> {
        HttpResponse::new("404", None, Self::load_build_in_file("404.html"))
    }
}

fn deal_file_resource(file_path: &str) -> HttpResponse<'static> {
    let file_content = fs::read_to_string(file_path);

    if file_content.is_err() {
        return HttpResponse::new("404", None, None);
    }

    let file_content = file_content.unwrap();

    let mut header = HashMap::new();
    if file_path.ends_with(".html") {
        header.insert("Content-Type", "text/html; charset=utf-8");
    } else if file_path.ends_with(".css") {
        header.insert("Content-Type", "text/css; charset=utf-8");
    } else if file_path.ends_with(".js") {
        header.insert("Content-Type", "text/javascript; charset=utf-8");
    } else if file_path.ends_with(".html") {
        header.insert("Content-Type", "text/html");
    } else if file_path.ends_with(".css") {
        header.insert("Content-Type", "text/css");
    } else if file_path.ends_with(".js") {
        header.insert("Content-Type", "text/javascript");
    } else if file_path.ends_with(".png") {
        header.insert("Content-Type", "image/png");
    } else if file_path.ends_with(".jpg") {
        header.insert("Content-Type", "image/jpeg");
    } else if file_path.ends_with(".gif") {
        header.insert("Content-Type", "image/gif");
    } else if file_path.ends_with(".ico") {
        header.insert("Content-Type", "image/x-icon");
    } else if file_path.ends_with(".svg") {
        header.insert("Content-Type", "image/svg+xml");
    } else if file_path.ends_with(".woff") {
        header.insert("Content-Type", "font/woff");
    } else if file_path.ends_with(".woff2") {
        header.insert("Content-Type", "font/woff2");
    } else if file_path.ends_with(".ttf") {
        header.insert("Content-Type", "font/ttf");
    } else if file_path.ends_with(".eot") {
        header.insert("Content-Type", "font/eot");
    } else if file_path.ends_with(".otf") {
        header.insert("Content-Type", "font/otf");
      } else if file_path.ends_with(".wasm") {
        header.insert("Content-Type", "application/wasm");
    } else if file_path.ends_with(".json") {
        header.insert("Content-Type", "application/json");
    } else if file_path.ends_with(".xml") {
        header.insert("Content-Type", "application/xml");
    } else if file_path.ends_with(".txt") {
        header.insert("Content-Type", "text/plain");
    } else if file_path.ends_with(".csv") {
        header.insert("Content-Type", "text/csv");
    } else if file_path.ends_with(".md") {
        header.insert("Content-Type", "text/markdown");
    } else if file_path.ends_with(".pdf") {
        header.insert("Content-Type", "application/pdf");
    } else if file_path.ends_with(".zip") {
        header.insert("Content-Type", "application/zip");
    } else if file_path.ends_with(".tar") {
        header.insert("Content-Type", "application/x-tar");
    } else if file_path.ends_with(".gz") {
        header.insert("Content-Type", "application/gzip");
    } else if file_path.ends_with(".bz2") {
        header.insert("Content-Type", "application/x-bzip2");
    } else {
        header.insert("Content-Type", "application/octet-stream");
    }
    HttpResponse::new("200", Some(header), Some(file_content))
}

fn deal_dir_resource(_resource: &str, _current_path: &str) -> HttpResponse<'static> {
    let read_dir = fs::read_dir(_resource).unwrap();

    let mut resources: Vec<String> = Vec::new();
    
    // 添加返回上级目录的链接（如果不是根目录）
    let mut navigation = String::new();
    if _current_path != "/" {
        let parent_path = if let Some(parent) = Path::new(_current_path).parent() {
            if parent.to_str().unwrap().is_empty() {
                "/".to_string()
            } else {
                parent.to_str().unwrap().to_string()
            }
        } else {
            "/".to_string()
        };
        navigation = format!("<div class='nav'><a href=\"{}\">← 返回上级目录</a></div>", parent_path);
    }
    
    for dir in read_dir {
        let dir_entry = dir.unwrap();
        let path = dir_entry.path();
        
        // 获取文件名或目录名
        let file_name = path.file_name().unwrap().to_str().unwrap();
        
        // 构建相对路径
        let relative_path = if _current_path == "/" {
            format!("/{}", file_name)
        } else {
            format!("{}/{}", _current_path, file_name)
        };

        // 判断是文件还是目录，添加不同的图标
        let icon = if path.is_dir() {
            "📁"
        } else {
            "📄"
        };

        let link_str = format!("<div class='item'><a href=\"{}\">{} {}</a></div>", relative_path, icon, file_name);
        resources.push(link_str);
    }
    
    let html_content = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>目录浏览 - {}</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 20px; background-color: #f5f5f5; }}
        .container {{ max-width: 800px; margin: 0 auto; background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }}
        .nav {{ margin-bottom: 20px; padding: 10px; background: #f8f9fa; border-radius: 4px; }}
        .nav a {{ color: #007bff; text-decoration: none; }}
        .nav a:hover {{ text-decoration: underline; }}
        h1 {{ color: #333; margin-bottom: 20px; }}
        .item {{ padding: 8px 12px; margin: 2px 0; border-radius: 4px; }}
        .item:hover {{ background-color: #f8f9fa; }}
        .item a {{ color: #333; text-decoration: none; }}
        .item a:hover {{ color: #007bff; }}
    </style>
</head>
<body>
    <div class="container">
        <h1>📂 目录浏览: {}</h1>
        {}
        <div class="content">
            {}
        </div>
    </div>
</body>
</html>"#,
        _current_path,
        _current_path,
        navigation,
        if resources.is_empty() { "<p>目录为空</p>".to_string() } else { resources.join("") }
    );

    let mut header = HashMap::new();
    header.insert("Content-Type", "text/html; charset=utf-8");
    HttpResponse::new("200", Some(header), Some(html_content))
}
