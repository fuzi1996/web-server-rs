use crate::handler::{Handler, StaticResourceHandler};
use http::httprequest::HttpRequest;
use http::httpresponse::HttpResponse;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

pub struct Route {}

impl Route {
    pub fn route(mut connection: TcpStream) {
        let buffer = BufReader::new(&connection);
        let request: HttpRequest = HttpRequest::from(
            buffer
                .lines()
                .map(|l| l.unwrap())
                .collect::<Vec<_>>()
                .join("\n"),
        );

        let path = request.resource_path();
        let mut response = HttpResponse::new("404", None, None);
        if path == "/" {
            response = StaticResourceHandler::handle_request(request);
        } else if path.ends_with(".html") {
            response = StaticResourceHandler::handle_request(request);
        }

        response.send_response(&mut connection);
    }
}
