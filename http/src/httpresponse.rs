use std::{collections::HashMap, fmt::format, io::Write, net::TcpStream};
use crate::httprequest::HttpVersion;

#[derive(Debug,PartialEq,Clone)]
pub struct HttpResponse<'a> {
  version: &'a HttpVersion,
  status_code: &'a str,
  status_text: &'a str,
  headers: Option<HashMap<&'a str, &'a str>>,
  body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
  fn default() -> Self {
    HttpResponse {
      version: &HttpVersion::HTTP11,
      status_code: "200",
      status_text: "OK",
      headers: None,
      body: None,
    }
  }
}

impl<'a> HttpResponse<'a> {
  pub fn new(status_code: &'a str, headers: Option<HashMap<&'a str, &'a str>>, body: Option<String>) -> HttpResponse<'a> {
    let mut response = HttpResponse::default();
    if status_code != "200" {
      response.status_code = status_code;
    }

    response.headers = match &headers {
      Some(_) => headers,
      None => {
        let mut h = HashMap::new();
        h.insert("Content-Type", "text/html");
        Some(h)
      }
    };

    response.body = body;

    match status_code {
      "200" => response.status_text = "OK",
      "404" => response.status_text = "Not Found",
      "500" => response.status_text = "Internal Server Error",
      _ => response.status_text = "Not Found",
    }
    response
  }

  pub fn send_response(&self, stream: &mut impl Write) -> Result<(), std::io::Error> {
    let response_string = String::from(self.clone());
    let _ = stream.write_all(response_string.as_bytes());
    Ok(())
  }

  pub fn version(&self) -> &HttpVersion {
    self.version
  }

  pub fn status_code(&self) -> &str {
    self.status_code
  }

  pub fn status_text(&self) -> &str {
    self.status_text
  }

  pub fn headers(&self) -> String { 
    let mut headers = String::new();
    if let Some(map) = self.headers.clone() {
      for (key, value) in map.iter() {
        headers = format!("{}{}: {}\r\n", headers,key, value);
      }
    }
    headers
  }

  pub fn body(&self) -> &str {
    match &self.body {
      Some(body) => body.as_str(),
      None => "",
    }
  }
  
}

impl<'a> From<HttpResponse<'a>> for String {
  fn from(response: HttpResponse) -> String {
    let response = response.clone();
    format!("{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}", &response.version(), &response.status_code(), &response.status_text(), &response.headers(),response.body().len(), &response.body())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::httprequest::HttpVersion;

  #[test]
  fn test_http_response_into_string() {
    let mut headers = HashMap::new();
    headers.insert("Content-Type", "text/html");
    let response = HttpResponse {
      version: &HttpVersion::HTTP11,
      status_code: "200",
      status_text: "OK",
      headers: Some(headers),
      body: Some("Hello, world!".to_string()),
    };
    let response_string: String = response.into();
    assert_eq!(response_string, "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: 13\r\n\r\nHello, world!");
  }

  #[test]
  fn test_http_response_into_string_without_headers() {
    let response = HttpResponse::default();
    let response_string: String = response.into();
    assert_eq!(response_string, "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n");
  }

  #[test]
  fn test_200_response() {
    let response = HttpResponse::new("200", None, Some("Hello, world!".to_string()));
    let response_string: String = String::from(response);
    assert_eq!(response_string, "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: 13\r\n\r\nHello, world!");
  }
}