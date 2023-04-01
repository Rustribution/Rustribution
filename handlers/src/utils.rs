use actix_web::{http, http::HeaderMap};

pub(crate) fn get_content_length(headers: &HeaderMap) -> usize {
    headers
        .get(http::header::CONTENT_LENGTH)
        .unwrap()
        .to_str()
        .unwrap_or("")
        .parse::<usize>()
        .unwrap_or(0)
}
