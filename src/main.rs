use tracing::info;
use app::routes::route;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = route::app();
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();
    info!("listening on {:?}", &listener);

    info!("server start");
    axum::serve(listener, app).await.unwrap();
}
