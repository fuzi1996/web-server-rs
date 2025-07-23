use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    UNINITIALIZED,
}

impl From<&str> for HttpMethod {
    fn from(method: &str) -> Self {
        match method {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            _ => HttpMethod::UNINITIALIZED,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum HttpVersion {
    HTTP10,
    HTTP11,
    UNINITIALIZED,
}

impl From<&str> for HttpVersion {
    fn from(version: &str) -> Self {
        match version {
            "HTTP/1.0" => HttpVersion::HTTP10,
            "HTTP/1.1" => HttpVersion::HTTP11,
            _ => HttpVersion::UNINITIALIZED,
        }
    }
}

impl From<&HttpVersion> for String {
    fn from(version: &HttpVersion) -> Self {
        match version {
            HttpVersion::HTTP10 => "HTTP/1.0".to_string(),
            HttpVersion::HTTP11 => "HTTP/1.1".to_string(),
            HttpVersion::UNINITIALIZED => "UNINITIALIZED".to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum HttpResource {
    PATH(String),
    UNINITIALIZED,
}

#[derive(Debug, PartialEq)]
pub struct HttpRequest {
    method: HttpMethod,
    resource: HttpResource,
    version: HttpVersion,
    headers: HashMap<String, String>,
    body: String,
}

impl HttpRequest {
    pub fn resource_path(&self) -> &str {
        match &self.resource {
            HttpResource::PATH(path) => path,
            HttpResource::UNINITIALIZED => "",
        }
    }
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = HttpMethod::UNINITIALIZED;
        let mut parsed_resource = HttpResource::UNINITIALIZED;
        let mut parsed_version = HttpVersion::UNINITIALIZED;
        let mut headers = HashMap::new();
        let mut body = String::new();

        for (index, line) in req.lines().enumerate() {
            if line.is_empty() {
                continue;
            }

            if index == 0 {
                let (method, resource, version) = process_resource(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.contains(":") {
                let (key, value) = process_header(line);
                headers.insert(key, value);
            } else if body.is_empty() {
                body.push_str(line);
            } else {
                body.push('\n');
                body.push_str(line);
            }
        }

        HttpRequest {
            method: parsed_method,
            resource: parsed_resource,
            version: parsed_version,
            headers,
            body,
        }
    }
}

fn process_resource(resource: &str) -> (HttpMethod, HttpResource, HttpVersion) {
    let parts: Vec<&str> = resource.split_whitespace().collect();
    if parts.len() == 3 {
        (
            parts[0].into(),
            HttpResource::PATH(parts[1].to_string()),
            parts[2].into(),
        )
    } else {
        (
            HttpMethod::UNINITIALIZED,
            HttpResource::UNINITIALIZED,
            HttpVersion::UNINITIALIZED,
        )
    }
}

fn process_header(header: &str) -> (String, String) {
    let parts: Vec<&str> = header.splitn(2, ':').collect();
    (parts[0].trim().to_string(), parts[1].trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_method_from_str() {
        let method: HttpMethod = "GET".into();
        assert_eq!(method, HttpMethod::GET);

        assert_eq!(HttpMethod::from("GET"), HttpMethod::GET);
        let method: HttpMethod = "POST".into();
        assert_eq!(method, HttpMethod::POST);
        let method: HttpMethod = "OPTIONS".into();
        assert_eq!(method, HttpMethod::UNINITIALIZED);
    }

    #[test]
    fn test_http_version_from_str() {
        let version: HttpVersion = "HTTP/1.0".into();
        assert_eq!(version, HttpVersion::HTTP10);

        assert_eq!(HttpVersion::from("HTTP/1.1"), HttpVersion::HTTP11);
        let version: HttpVersion = "HTTP/2.0".into();
        assert_eq!(version, HttpVersion::UNINITIALIZED);
    }

    #[test]
    fn test_http_request_from_str() {
        let request: HttpRequest = "GET / HTTP/1.1\r\nHost: localhost:8080\r\nContent-Length: 10\r\n\r\nHello, world!\nEnd Line.".to_string().into();
        assert_eq!(request.method, HttpMethod::GET);
        assert_eq!(request.resource, HttpResource::PATH("/".to_string()));
        assert_eq!(request.version, HttpVersion::HTTP11);
        assert_eq!(
            request.headers.get("Host"),
            Some(&"localhost:8080".to_string())
        );
        assert_eq!(request.headers.len(), 2);
        assert_eq!(request.body, "Hello, world!\nEnd Line.");
    }
}
