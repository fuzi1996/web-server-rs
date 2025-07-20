use std::{net::TcpListener, path::Path};
use log::{error, info};
use threadpool::threadpool::ThreadPool;
use crate::route::Route;

pub struct HttpServer<'a> {
    host: &'a str,
    port: u16,
    work_dir: &'a str,
}

impl<'a> HttpServer<'a> {
    pub fn new(host: &'a str, port: u16, work_dir: &'a str) -> Self {
        Self {
            host,
            port,
            work_dir,
        }
    }

    pub fn run(&self) {
        let listener = match TcpListener::bind(format!("{}:{}", self.host, self.port)) {
            Ok(listener) => listener,
            Err(e) => {
                error!("Failed to bind to port {}: {}", self.port, e);
                std::process::exit(1);
            }
        };

        self.set_work_dir();

        let pool = ThreadPool::new(4);

        info!(
            "Server is running on http://{}:{} in {}",
            self.host, self.port, self.work_dir
        );
        for stream in listener.incoming() {
            pool.execute(move || {
                let connection = stream.unwrap();
                Route::route(connection);
            });
        }
    }

    fn set_work_dir(&self) {
        if !Path::new(self.work_dir).exists() {
            error!("Work directory {} does not exist", self.work_dir);
            std::process::exit(1);
        }
        std::env::set_current_dir(self.work_dir).unwrap();
    }
}
