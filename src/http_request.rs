#[derive(Debug)]
pub enum Method {
    PUT,
    GET,
    HEAD,
    POST,
}

pub async fn get_method(fullstring: &str) -> (Option<Method>, &str) {
    let (request, tail) = match fullstring.split_once(' ') {
        Some(n) => n,
        None => ("", ""),
    };
    let method = match request {
        "GET" => Some(Method::GET),
        "PUT" => Some(Method::PUT),
        "HEAD" => Some(Method::HEAD),
        "POST" => Some(Method::POST),
        _ => None,
    };
    (method, tail)
}
