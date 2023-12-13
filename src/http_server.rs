use std::{net::SocketAddr, sync::Arc};

use my_http_server::MyHttpServer;

use crate::app_ctx::AppContext;

pub fn start(app: &AppContext) {
    let mut http_server = MyHttpServer::new(SocketAddr::from(([0, 0, 0, 0], 8080)));

    http_server.add_middleware(Arc::new(my_http_server::StaticFilesMiddleware::new(
        None,
        Some(vec!["Index.html".to_string()]),
    )));

    http_server.start(app.states.clone(), my_logger::LOGGER.clone());
}
