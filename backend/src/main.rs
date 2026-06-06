use axum::{extract::Path, response::Html, routing::get, Router};

async fn home() -> Html<&'static str> {
    Html("<h1>Home Page</h1>")
}

async fn post_page(Path(slug): Path<String>) -> Html<String> {
    Html(format!("<h1>Post Page</h1><p>Slug: {slug}</p>"))
}

async fn dashboard_auth() -> Html<&'static str> {
    Html("<h1>Dashboard / Auth Page</h1>")
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/post/{slug}", get(post_page))
        .route("/dashboard/auth", get(dashboard_auth));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

