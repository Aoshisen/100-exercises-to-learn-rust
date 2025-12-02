// TODO: Implement the `fixed_reply` function. It should accept two `TcpListener` instances,
//  accept connections on both of them concurrently, and always reply to clients by sending
//  the `Display` representation of the `reply` argument as a response.
use std::fmt::Display;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

pub async fn fixed_reply<T>(first: TcpListener, second: TcpListener, reply: T)
where
    // `T` cannot be cloned. How do you share it between the two server tasks?
    T: Display + Send + Sync + 'static,
{
    // 使用 Arc 包装 reply 以便在线程间共享
    let shared_reply = Arc::new(reply);
    loop {
        tokio::select! {
            result = first.accept() => {
                match result {
                    Ok((socket, _)) => {
                        let reply_clone=shared_reply.clone();
                        tokio::spawn(async move {
                            if let Err(e) = handle_connection(socket,reply_clone).await {
                                eprintln!("Error handling connection: {e}");
                            }
                        });
                    }
                    Err(e) => {
                        eprintln!("Error accepting connection on first listener: {e}");
                    }
                }
            }
            result = second.accept() => {
                match result {
                    Ok((socket, _)) => {
                        let reply_clone=shared_reply.clone();
                        tokio::spawn(async move {
                            if let Err(e) = handle_connection(socket,reply_clone).await {
                                eprintln!("Error handling connection: {e}");
                            }
                        });
                    }
                    Err(e) => {
                        eprintln!("Error accepting connection on second listener: {e}");
                    }
                }
            }
        }
    }
}

async fn handle_connection<T>(
    mut socket: tokio::net::TcpStream,
    reply: T,
) -> Result<(), anyhow::Error>
where
    T: Display + Sync,
{
    let response = format!("{reply}");
    socket.write_all(response.as_bytes()).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::panic;
    use tokio::io::AsyncReadExt;
    use tokio::task::JoinSet;

    async fn bind_random() -> (TcpListener, SocketAddr) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        (listener, addr)
    }

    #[tokio::test]
    async fn test_echo() {
        let (first_listener, first_addr) = bind_random().await;
        let (second_listener, second_addr) = bind_random().await;
        let reply = "Yo";
        tokio::spawn(fixed_reply(first_listener, second_listener, reply));

        let mut join_set = JoinSet::new();

        for _ in 0..3 {
            for addr in [first_addr, second_addr] {
                join_set.spawn(async move {
                    let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
                    let (mut reader, _) = socket.split();

                    // Read the response
                    let mut buf = Vec::new();
                    reader.read_to_end(&mut buf).await.unwrap();
                    assert_eq!(&buf, reply.as_bytes());
                });
            }
        }

        while let Some(outcome) = join_set.join_next().await {
            if let Err(e) = outcome {
                if let Ok(reason) = e.try_into_panic() {
                    panic::resume_unwind(reason);
                }
            }
        }
    }
}
