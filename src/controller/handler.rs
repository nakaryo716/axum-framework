use axum::response::{Html, IntoResponse};

pub async fn hello() -> impl IntoResponse {
    Html("<h1>Hello World!</h1>")
}
