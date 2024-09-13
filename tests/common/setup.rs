use std::sync::{Once, OnceLock};

use actix_web::dev::ServerHandle;
use file_graph::{init_logger, init_server};
use rstest::*;

static INIT: Once = Once::new();
#[fixture]
pub fn setup() -> () {
    INIT.call_once(|| {
        init_logger();
    });
}

#[fixture]
pub async fn test_server() -> &'static ServerHandle {
    static SERVER: OnceLock<ServerHandle> = OnceLock::new();
    let handler = SERVER.get_or_init(|| {
        init_logger();

        let server = init_server().unwrap();
        let server_handler = server.handle();
        let server_spawner = Box::new(server);
        tokio::spawn(async move {
            server_spawner.await.unwrap();
        });
        server_handler
    });
    handler.resume().await;
    handler
}
