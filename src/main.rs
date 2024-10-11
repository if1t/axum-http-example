use axum::{
    response::Html,
    routing::get,
    Router,
    http::{StatusCode},
    extract::Query,
};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello", get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(serde::Deserialize)]
struct QueryParams {
    name: String,
}

async fn handler(Query(params): Query<QueryParams>) -> Result<Html<String>, StatusCode> {
    let name = params.name;
    Ok(Html(String::from(format!("Hello, {}!", name))))
}
