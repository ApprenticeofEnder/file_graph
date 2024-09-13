use std::{future::Future, panic, pin::Pin, sync::Once};

use file_graph::{init_logger, init_server};
use rstest::*;

static INIT: Once = Once::new();
#[fixture]
pub fn setup() -> () {
    INIT.call_once(|| {
        init_logger();
    });
}

pub fn run_test<T>(test: T) -> ()
where
    T: panic::UnwindSafe,
    T: FnOnce() -> Pin<Box<dyn Future<Output = ()> + 'static + Send>>,
{
    let result = std::panic::catch_unwind(|| {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                // spawning an app now returns a tuple so we can move `test_app` into the test
                // function while keeping everything we need to later delete the database

                let server = init_server().unwrap();

                let server_handler = server.handle();
                let server_spawner = Box::new(server);
                tokio::spawn(async move {
                    server_spawner.await.unwrap();
                });
                // run some tests on the newly started app !
                test().await;
                server_handler.stop(true).await;
            })
    });
    assert!(result.is_ok());
}
