// TODO: fix the `assert_eq` at the end of the tests.
//  Do you understand why that's the resulting output?
use std::time::Duration;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

pub async fn run(listener: TcpListener, n_messages: usize, timeout: Duration) -> Vec<u8> {
    let mut buffer = Vec::new();
    for _ in 0..n_messages {
        let (mut stream, _) = listener.accept().await.unwrap();
        let _ = tokio::time::timeout(timeout, async {
            //我在20 的时候打开这个stream 接受数据
            //读缓冲区的数据到buffer
            stream.read_to_end(&mut buffer).await.unwrap();
            //await 我在20 的时候写完到buffer 之后我这个stream 就随着 timeout 关闭掉了 后面的时间我就不干事了
        }).await;
    }
    //读传过来的所有的数据然后原封不动的返回timeout 这么多时间再读取
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn ping() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let messages = vec!["hello", "from", "this", "task"];
        let timeout = Duration::from_millis(20);
        let handle = tokio::spawn(run(listener, messages.len(), timeout.clone()));

        for message in messages {
            let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
            let (_, mut writer) = socket.split();

            let (beginning, end) = message.split_at(message.len() / 2);

            // Send first half
            //0.相当于我在0 的时候发送数据
            writer.write_all(beginning.as_bytes()).await.unwrap();
            //睡了40
            tokio::time::sleep(timeout * 2).await;
            //在40 的时候再给run 发消息,因为20 的时候run 里面被关闭掉了不会执行写入逻辑
            writer.write_all(end.as_bytes()).await.unwrap();

            // Close the write side of the socket
            let _ = writer.shutdown().await;
        }

        let buffered = handle.await.unwrap();
        let buffered = std::str::from_utf8(&buffered).unwrap();
        //hefrthta
        assert_eq!(buffered, "hefrthta");
    }
}
