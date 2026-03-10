use std::{net::SocketAddr, sync::Arc};

use my_http_server::MyHttpServer;

use crate::app_ctx::AppContext;

pub fn start(app: &AppContext) {
    let mut http_server = MyHttpServer::new(SocketAddr::from(([0, 0, 0, 0], 8082)));

    let static_files = my_http_server::StaticFilesMiddleware::new(
        
    ).add_index_file("Index.html");

    http_server.add_middleware(Arc::new(static_files));

    http_server.start(app.states.clone(), my_logger::LOGGER.clone());
}
