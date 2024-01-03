//! Command serve

use crate::{cmd::Watch, LIVERELOAD_ENDPOINT};
use anyhow::Result;
use async_lock::Mutex;
use ccli::{clap, clap::Parser};
use futures::{sink::SinkExt, FutureExt, StreamExt};
use notify::Event;
use std::{
    net::{Ipv4Addr, TcpListener},
    sync::{
        mpsc::{self, Receiver},
        Arc,
    },
};
use tokio::runtime::Runtime;
use warp::{
    ws::{Message, WebSocket, Ws},
    Filter,
};

/// Serve command
#[derive(Parser, Debug)]
pub struct Serve {
    /// Port to listen on
    #[clap(short, long, default_value = "3000")]
    pub port: u16,

    /// Address to listen on
    #[clap(short, long, default_value = "0.0.0.0")]
    pub address: Ipv4Addr,

    /// Watch configuration
    #[clap(flatten)]
    pub watch: Watch,
}

impl Serve {
    /// Pick a port for the livereload server
    fn pick(&self) -> u16 {
        let mut base = self.port;
        loop {
            if TcpListener::bind((self.address, base)).is_ok() {
                return base;
            }

            base += 1;
        }
    }

    /// Run the serve command
    pub fn run(&self) -> Result<()> {
        let port = self.pick();

        let (tx, rx) = mpsc::channel::<Event>();
        let rx = Arc::new(Mutex::new(rx));
        let livereload = warp::path(LIVERELOAD_ENDPOINT)
            .and(warp::ws())
            .and(warp::any().map(move || rx.clone()))
            .map(|ws: Ws, rx: Arc<Mutex<Receiver<Event>>>| {
                ws.on_upgrade(move |socket: WebSocket| async move {
                    let (mut tx, _) = socket.split();
                    let rx = rx.lock().await;
                    if rx.recv().is_ok() {
                        if let Err(e) = tx.send(Message::text("reload")).await {
                            tracing::error!("failed to send reload message: {}", e);
                        }
                    }
                })
            });

        let manifest = self.watch.manifest()?;
        let watcher = self.watch.clone();
        let service = warp::serve(warp::fs::dir(manifest.out.clone()).or(livereload))
            .run((self.address, port));
        Runtime::new()?.block_on(async {
            tracing::info!("listening on http://{}:{} ...", self.address, port);
            let watcher = tokio::task::spawn_blocking(move || watcher.watch(manifest, tx));

            if let Err(e) = futures::select! {
                r = service.fuse() => Ok(r),
                r = watcher.fuse() => r.map_err(Into::into).and_then(|r| r),
            } {
                tracing::error!("failed to run server: {}", e);
            }
        });
        Ok(())
    }
}
