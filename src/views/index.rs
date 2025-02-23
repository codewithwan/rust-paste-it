use axum::response::Html;
use std::fs;

pub async fn index() -> Html<String> {
    let index_template = fs::read_to_string("src/views/templates/index.html")
        .expect("Failed to read index.html template");
    Html(index_template)
}

pub async fn not_found() -> Html<String> {
    let template = fs::read_to_string("src/views/templates/404.html")
        .expect("Failed to read 404.html template");
    Html(template)
}

pub async fn api_docs() -> Html<String> {
    let template = fs::read_to_string("src/views/templates/api-docs.html")
        .expect("Failed to read api-docs.html template");
    Html(template)
}
