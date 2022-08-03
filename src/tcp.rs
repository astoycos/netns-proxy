use std::net::SocketAddr;

use anyhow::{Context, Result};
use async_trait::async_trait;
use tokio::net::{TcpListener, TcpStream};

pub struct Proxy {
    socket:  TcpListener,
}

#[async_trait]
impl super::Proxy for Proxy {
    async fn listen(bind: SocketAddr) -> Result<Self> {
        let socket = TcpListener::bind(bind).await
            .with_context(|| format!("Failed to bind to TCP socket: {}", bind))?;
        return Ok(Self {
            socket
        })
    }

    async fn run(mut self: Box<Self>, target: SocketAddr) -> Result<()> {        
        loop {
            let (mut client, _) = self.socket.accept().await?;

            tokio::spawn(async move {
                // In a loop, read data from the socket and write the data back.
                loop {
                    let mut remote = match TcpStream::connect(target).await {
                        Ok(n) => n,
                        Err(e) => {
                            eprintln!("Failed to connect to target; err = {:?}", e);
                            return;
                        }
                    };

                    if let Err(e) = tokio::io::copy_bidirectional(&mut client, &mut remote).await{
                        eprintln!("failed to copy from/to proxy; err = {:?}", e);
                        return;
                    }

                    return;
                }
            });
        }
    }
}