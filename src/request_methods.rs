#[derive(Debug)]
pub enum Method {
    PUT,
    GET,
    HEAD,
    POST,
}

pub async fn return_method(fullstring: &str) -> Option<Method> {
    let request = &fullstring[0..fullstring.find(' ').unwrap()];
    match request {
        "GET" => Some(Method::GET),
        "PUT" => Some(Method::PUT),
        "HEAD" => Some(Method::HEAD),
        "POST" => Some(Method::POST),
        _ => None,
    }
}
