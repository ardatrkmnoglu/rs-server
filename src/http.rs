pub struct Request {
    path: String,
    query: Option<String>,
    method: Method,
}

pub enum Method {
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}
