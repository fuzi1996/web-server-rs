use std::env::{self, args};

use httpserver::httpserver::HttpServer;
use log::LevelFilter;

fn main() {
    init_log();

    // 如果参数为 -h --help 打印帮助信息
    if args().nth(1).unwrap_or("".to_string()) == "-h" || args().nth(1).unwrap_or("".to_string()) == "--help" {
        println!("Usage: httpserver [port] [work_dir]");
        println!("Usage: httpserver [options]");
        println!("Options:");
        println!("  -h, --help     Print this help message");
        println!("  -v, --version  Print the version number");
        return;
    }

    // 如果参数为 -v --version 打印版本信息
    if args().nth(1).unwrap_or("".to_string()) == "-v" || args().nth(1).unwrap_or("".to_string()) == "--version" {
        let version = env!("CARGO_PKG_VERSION");
        let name = env!("CARGO_PKG_NAME");
        println!("{name} {version}");
        return;
    }

    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = args().nth(1).unwrap_or("7878".to_string());
    // let work_dir = args().nth(2).unwrap_or(".".to_string());
    let work_dir = "C:\\Users\\11829\\repo\\trpl-zh-cn\\book";

    let server = HttpServer::new(&host, port.parse().unwrap(), work_dir);
    server.run();
}

fn init_log() {
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
}
