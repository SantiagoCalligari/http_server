#[derive(Debug)]
pub struct HttpRequest {
    pub method: Option<String>,
    pub path: String,
    pub version: String,
    pub headers: Option<Vec<Header>>,
}
#[derive(Debug)]
pub struct Header {
    pub key: String,
    pub value: String,
}

pub async fn parse_header(full_header: &str) -> Header {
    let mut strings = full_header.split_whitespace();

    Header {
        key: strings.next().unwrap().to_string(),
        value: strings.next().unwrap().to_string(),
    }
}

pub async fn parse_request(raw_request: &str) -> HttpRequest {
    let mut strings = raw_request.lines();
    let (method, path, version) = get_method_path_version(strings.next().unwrap()).await;
    for line in strings {
        parse_header(line);
    }
    let request = HttpRequest {
        method: method,
        path: path,
        version: version,
        headers: None,
    };

    request
}
pub async fn get_method_path_version(fullstring: &str) -> (Option<String>, String, String) {
    let mut separated = fullstring.split_whitespace();
    let method = match separated.next() {
        Some(n) => Some(n.to_string()),
        None => None,
    };
    let path = separated.next().unwrap().to_string();
    let version = separated.next().unwrap().to_string();
    (method, path, version)
}
