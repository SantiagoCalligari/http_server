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

pub async fn parse_header(full_header: &str) -> Header {
    let mut strings = full_header.split_whitespace();

    Header {
        key: strings.next().unwrap_or(""),
        value: strings.next().unwrap_or(""),
    }
}

pub async fn parse_request(raw_request: &str) -> HttpRequest {
    let mut strings = raw_request.lines();
    let (method, path, version) = get_method_path_version(strings.next().unwrap()).await;
    let mut headers: Vec<Header> = Vec::new();
    for line in strings {
        headers.push(parse_header(line).await);
    }
    let request = HttpRequest {
        method: method,
        path: path,
        version: version,
        headers: headers,
    };

    request
}
pub async fn get_method_path_version(fullstring: &str) -> (&str, &str, &str) {
    let mut split = fullstring.split_whitespace();
    (
        split.next().unwrap_or(""),
        split.next().unwrap_or(""),
        split.next().unwrap_or(""),
    )
}
