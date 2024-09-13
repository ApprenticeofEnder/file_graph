use file_graph::{init_logger, init_server};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    init_logger();
    init_server()?.await
}
