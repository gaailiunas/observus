use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::{Level, debug, error, trace};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    let mut buf = [0; 1024];
    loop {
        let n = match stream.read(&mut buf).await {
            Ok(0) => {
                debug!("Socket closed");
                continue;
            }
            Ok(n) => {
                debug!("Read {} bytes", n);
                trace!("Data: {:?}", &buf[..n]);
                n
            }
            Err(e) => {
                error!("failed to read from socket; err = {:?}", e);
                continue;
            }
        };
    }
}
