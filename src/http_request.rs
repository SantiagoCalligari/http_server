#[derive(Debug)]
pub struct HttpRequest<'a> {
    pub method: &'a str,
    pub path: &'a str,
    pub version: &'a str,
    pub headers: Vec<Header<'a>>,
}

#[derive(Debug)]
pub struct Header<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

async fn parse_header(full_header: &str) -> Header {
    let mut strings = full_header.split_whitespace();

    Header {
        key: strings.next().unwrap_or(""),
        value: strings.next().unwrap_or(""),
    }
}

pub async fn parse_request(raw_request: &str) -> HttpRequest {
    let mut headers: Vec<Header> = Vec::new();
    let mut strings = raw_request.lines().peekable();
    let (method, path, version) = get_method_path_version(strings.next().unwrap()).await;

    while let Some(line) = strings.next() {
        if !strings.peek().is_none() {
            let header = parse_header(line).await;
            headers.push(header);
        }
    }

    HttpRequest {
        method: method,
        path: path,
        version: version,
        headers: headers,
    }
}

async fn get_method_path_version(fullstring: &str) -> (&str, &str, &str) {
    let mut split = fullstring.split_whitespace();
    let method = split.next().unwrap_or("");
    let mut path = &split.next().unwrap_or("")[1..];
    if path == "" {
        path = "index"
    }
    let version = split.next().unwrap_or("");

    (method, path, version)
}
