use std::collections::HashMap;
use std::{env, fs};
use std::path::Path;

use http::httprequest::HttpRequest;
use http::httpresponse::HttpResponse;
use log::info;

pub trait Handler {
    fn handle_request(request: HttpRequest) -> HttpResponse<'static>;

    fn load_build_in_file(file_path: &str) -> Option<String> {
        let default_path = format!("{}/static", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let file_path = format!("{}/{}", public_path, file_path);
        if Path::new(&file_path).exists() {
            return Some(file_path);
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
            return deal_dir_resource(path);
        }

        match fs::read_to_string(&file_path) {
            Ok(file_content) => {
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
            Err(_) => NotFoundHandler::handle_request(request),
        }
    }
}

pub struct NotFoundHandler {}

impl Handler for NotFoundHandler {
    fn handle_request(_: HttpRequest) -> HttpResponse<'static> {
        HttpResponse::new("404", None, Self::load_build_in_file("404.html"))
    }
}

fn deal_file_resource(_resource: &str) -> HttpResponse<'static> {
    let file_content = fs::read_to_string(_resource).unwrap();
    let mut header = HashMap::new();
    header.insert("Content-Type", "text/html");
    HttpResponse::new("200", Some(header), Some(file_content))
}

fn deal_dir_resource(_resource: &str) -> HttpResponse<'static> {
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

    let mut header = HashMap::new();
    header.insert("Content-Type", "text/html");
    HttpResponse::new("200", Some(header), Some(resource_content))
}
