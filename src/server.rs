use crate::*;
use color_print::cformat;
use futures::{SinkExt, StreamExt};
use std::{error::Error, net::SocketAddr, sync::Arc, time::Duration};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{Barrier, Mutex},
    time::sleep,
};
use tokio_util::codec::{Framed, LinesCodec};

#[derive(Clone)]
pub struct Server {
    pub own_address: String,
    pub number_of_peers: usize,
}

pub struct PeerConnecton {
    pub address: SocketAddr,
}

impl Server {
    pub fn new(own_address: String, number_of_peers: usize) -> Self {
        Self {
            own_address,
            number_of_peers,
        }
    }

    async fn handle(
        stream: TcpStream,
        address: SocketAddr,
        _server: Arc<Mutex<Self>>,
        barrier: Arc<Barrier>,
        starts_with_hot_potato: Arc<Mutex<bool>>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let lines = Framed::new(stream, LinesCodec::new());
        let (mut writer, mut reader) = lines.split::<String>();

        let mut participant = PeerConnecton { address };

        // wait for all participants to join
        barrier.wait().await;

        log::info("send start flag to peer.");
        writer.send(StartFlag(true).to_json_string()?).await?;
        writer.flush().await?;

        {
            let mut starts_with_hot_potato = starts_with_hot_potato.lock().await;

            if *starts_with_hot_potato {
                log::info(&cformat!(
                    "Sending <yellow, bold>hot potato</yellow, bold> to a peer."
                ));
                *starts_with_hot_potato = false;
                writer.send(StartFlag(true).to_json_string()?).await?;
                writer.flush().await?;
            }
        }

        loop {
            sleep(Duration::from_secs(5)).await;
        }
    }

    pub async fn run(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let listener = TcpListener::bind(&self.own_address).await?;
        let server = Arc::new(Mutex::new(self.clone()));

        let barrier = Arc::new(Barrier::new(self.number_of_peers));
        let starts_with_hot_potato = Arc::new(Mutex::new(true));

        loop {
            let (peer_stream, peer_address) = listener.accept().await?;

            log::info("Accepted a connection.");

            let server = server.clone();
            let barrier = barrier.clone();
            let starts_with_hot_potato = starts_with_hot_potato.clone();

            let _handle = tokio::spawn(async move {
                if let Err(e) = Self::handle(
                    peer_stream,
                    peer_address,
                    server,
                    barrier,
                    starts_with_hot_potato,
                )
                .await
                {
                    log::error("{e}");
                };
            });
        }
    }
}
