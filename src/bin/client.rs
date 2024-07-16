use bytes::BytesMut;
use tokio::{
    io::{stdin, stdout, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    select,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:3000").await?;
    let mut stdin = stdin();
    let mut stdout = stdout();
    let mut net_buf = BytesMut::new();
    let mut loc_buf = BytesMut::new();
    loop {
        select! {
            len =  stream.read_buf(&mut net_buf) => {
                if len.is_err() || len.unwrap() == 0 {
                    break;
                }
                stdout.write_buf(&mut net_buf).await?;
                net_buf.clear();
            },
            len = stdin.read_buf(&mut loc_buf) => {
                if len.is_err() || len.unwrap() == 0 {
                    break;
                }
                stream.write_buf(&mut loc_buf).await?;
                loc_buf.clear();
            }
        }
    }
    Ok(())
}
