use std::clone;

use bytes::Bytes;
use mini_redis::client;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio::sync::oneshot::Receiver;
use tokio::sync::oneshot::Sender;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:6379";
    let (tx, mut rx): (mpsc::Sender<Command>, mpsc::Receiver<Command>) = mpsc::channel(32);
    
    
    // The manager thread holds the current redis connection and processes commands coming from different threads.
    let manager = tokio::spawn(async move {
        let mut client = client::connect(addr).await.unwrap();

        while let Some(cmd) = rx.recv().await {
            use Command::*;

            match cmd {
                Get { key, recv } => {
                    client.get(&key).await.unwrap();
                }
                Set { key, val, recv } => {
                    client.set(&key, val).await.unwrap();
                }
            }
        }
    });

    // tx1 and tx2 are producers and send requests to be processded by the manager thread above.
    let tx1 = tx.clone();
    let tx2 = tx.clone();

    todo!();
}

#[derive(Debug)]
enum Command {
    Get { key: String, recv: oneshot::Receiver<Option<Bytes>>},
    Set { key: String, val: Bytes, recv: oneshot::Receiver<()> },
}
