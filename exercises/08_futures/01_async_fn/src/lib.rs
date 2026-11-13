use tokio::net::TcpListener;
use tokio::io::{ AsyncReadExt, AsyncWriteExt };

// TODO: write an echo server that accepts incoming TCP connections and
//  echoes the received data back to the client.
//  `echo` should not return when it finishes processing a connection, but should
//  continue to accept new connections.
//
// Hint: you should rely on `tokio`'s structs and methods to implement the echo server.
// In particular:
// - `tokio::net::TcpListener::accept` to process the next incoming connection
// - `tokio::net::TcpStream::split` to obtain a reader and a writer from the socket
// - `tokio::io::copy` to copy data from the reader to the writer
pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("new connection:{addr}");

        tokio::spawn(async move {
            let mut buf = [0u8; 4096];
            loop {
                //时刻2 服务端被唤醒 在接收缓冲区取数据
                match socket.read(&mut buf).await {
                    Ok(0) => {
                        //没取到数据那么直接break
                        break;
                    }
                    Ok(n) => {
                        //取到了数据 直接塞到内核的发送缓冲区
                        if socket.write_all(&buf[..n]).await.is_err() {
                            break;
                        }
                    }

                    Err(e) => {
                        eprintln!("read Error{}", e);
                        break;
                    }
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{ AsyncReadExt, AsyncWriteExt };

    #[tokio::test]
    async fn test_echo() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(echo(listener));

        let requests = vec!["hello", "world", "foo", "bar"];

        for request in requests {
            let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
            let (mut reader, mut writer) = socket.split();

            // Send the request
            //把数据拷进操作系统内核的发送缓冲区
            //内核负责吧这数据通过loopback 网络发送到对端的socket的接收缓冲区
            writer.write_all(request.as_bytes()).await.unwrap();
            // Close the write side of the socket
            writer.shutdown().await.unwrap();

            // Read the response
            let mut buf = Vec::with_capacity(request.len());
            reader.read_to_end(&mut buf).await.unwrap(); //从客户端的接收缓冲区取
            assert_eq!(&buf, request.as_bytes());
        }
    }
}
