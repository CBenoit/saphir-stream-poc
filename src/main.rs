use saphir::prelude::*;

#[tokio::main]
async fn main() -> Result<(), SaphirError> {
    env_logger::init();

    let server = Server::builder()
        .configure_listener(|l| l.interface("127.0.0.1:4749").server_name("MagicService"))
        .configure_router(|r| r.route("/", Method::POST, magic_handler))
        .build();

    server.run().await
}

async fn magic_handler(mut req: Request) -> u16 {
    use hyper::body::HttpBody; // data()

    log::info!("Entered magic_handler");
    let mut body: hyper::Body = req.body_mut().take().into();
    log::info!("Converted into `hyper::Body`");

    while let Some(chunk_res) = body.data().await {
        match chunk_res {
            Ok(chunk) => log::info!("Got a chunk [len = {}]", chunk.len()),
            Err(e) => {
                log::error!("Failed to read chunk [{}]", e);
                return 400;
            }
        }
    }

    log::info!("Exited magic_handler");
    200
}
