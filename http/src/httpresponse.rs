use crate::httprequest::HttpVersion;
use std::{collections::HashMap, fmt::Debug, io::Write};

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a HttpVersion,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
    binary_body: Option<Vec<u8>>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        HttpResponse {
            version: &HttpVersion::HTTP11,
            status_code: "200",
            status_text: "OK",
            headers: None,
            body: None,
            binary_body: None,
        }
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> HttpResponse<'a> {
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

    pub fn new_binary(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        binary_body: Option<Vec<u8>>,
    ) -> HttpResponse<'a> {
        let mut response = HttpResponse::default();
        if status_code != "200" {
            response.status_code = status_code;
        }

        response.headers = headers;
        response.binary_body = binary_body;

        match status_code {
            "200" => response.status_text = "OK",
            "404" => response.status_text = "Not Found",
            "500" => response.status_text = "Internal Server Error",
            _ => response.status_text = "Not Found",
        }
        response
    }

    pub fn send_response(mut self, stream: &mut impl Write) -> Result<(), std::io::Error> {
        if let Some(binary_body) = self.binary_body.take() {
            // 发送二进制响应
            let response_string = self.to_binary_response_string(&binary_body);
            stream.write_all(response_string.as_bytes())?;
            stream.write_all(&binary_body)?;
            stream.flush()?;
        } else {
            // 发送文本响应
            let response_string = String::from(self);
            stream.write_all(response_string.as_bytes())?;
            stream.flush()?;
        }
        Ok(())
    }

    fn to_binary_response_string(&self, binary_body: &[u8]) -> String {
        let mut headers = String::new();
        if let Some(map) = &self.headers {
            for (key, value) in map.iter() {
                headers = format!("{headers}{key}: {value}\r\n");
            }
        }
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n",
            &self.version(),
            &self.status_code(),
            &self.status_text(),
            headers,
            binary_body.len()
        )
    }

    pub fn version(&self) -> String {
        String::from(self.version)
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
                headers = format!("{headers}{key}: {value}\r\n");
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

    pub fn binary_body(&self) -> Option<&[u8]> {
        self.binary_body.as_deref()
    }
}

impl<'a> From<HttpResponse<'a>> for String {
    fn from(response: HttpResponse<'a>) -> String {
        let response = response.clone();
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            &response.version(),
            &response.status_code(),
            &response.status_text(),
            &response.headers(),
            response.body().len(),
            &response.body()
        )
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
            binary_body: None,
        };
        let response_string: String = response.into();
        assert_eq!(
            response_string,
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: 13\r\n\r\nHello, world!"
        );
    }

    #[test]
    fn test_http_response_into_string_without_headers() {
        let response = HttpResponse::default();
        let response_string: String = response.into();
        assert_eq!(
            response_string,
            "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n"
        );
    }

    #[test]
    fn test_200_response() {
        let response = HttpResponse::new("200", None, Some("Hello, world!".to_string()));
        let response_string: String = String::from(response);
        assert_eq!(
            response_string,
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: 13\r\n\r\nHello, world!"
        );
    }

    #[test]
    fn test_404_response() {
        let response = HttpResponse::new("404", None, Some("Not Found".to_string()));
        let response_string: String = String::from(response);
        assert_eq!(
            response_string,
            "HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\nContent-Length: 9\r\n\r\nNot Found"
        );
    }

    #[test]
    fn test_500_response() {
        let response = HttpResponse::new("500", None, Some("Internal Server Error".to_string()));
        let response_string: String = String::from(response);
        assert_eq!(
            response_string,
            "HTTP/1.1 500 Internal Server Error\r\nContent-Type: text/html\r\nContent-Length: 21\r\n\r\nInternal Server Error"
        );
    }
}
