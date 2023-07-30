use tokio::io;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7777").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let (mut r, mut w) = socket.split();

            if io::copy(&mut r, &mut w).await.is_err() {
                eprintln!("failed to copy");
            }
        });
    }
}
