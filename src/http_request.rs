#[derive(Debug)]
pub struct HttpRequest {
    pub method: Some(Method),
    pub path: Some(String),
    pub version: Some(String),
    pub headers: Some(Vec<Header>),
}
#[derive(Debug)]
pub struct Header{
    pub key: String,
    pub value: String,
}
#[derive(Debug)]
pub enum Method {
    PUT,
    GET,
    HEAD,
    POST,
}



pub async fn parse_request(raw_request: &str) -> HttpRequest {
    let mut request:HttpRequest;
    let strings = raw_request.lines();
    (request.method, request.path, request.version) = get_method_path_version(strings.next());
    
    request
}
pub async fn get_method_path_version(fullstring: &str) -> (Option<Method>, String, String) {
    let separated = fullstring.split_whitespace();
    let method = match separated.next() {
        "GET" => Some(Method::GET),
        "PUT" => Some(Method::PUT),
        "HEAD" => Some(Method::HEAD),
        "POST" => Some(Method::POST),
        _ => None,
    };
    let path = separated.next();
    let version = separated.next();
    (method, path, version)
}
