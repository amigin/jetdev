mod app_ctx;
mod http_server;
use app_ctx::AppContext;

#[tokio::main]
async fn main() {
    let app = AppContext::new();

    crate::http_server::start(&app);

    app.states.wait_until_shutdown().await;
}
