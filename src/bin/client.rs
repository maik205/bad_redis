use bytes::Bytes;
use mini_redis::client;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

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
                Get { key, replier } => {
                    replier.send(client.get(&key).await.unwrap()).unwrap();
                }
                Set { key, val, replier } => {
                    client.set(&key, val).await.unwrap();
                    replier.send(()).unwrap();
                }
            }
        }
    });

    // tx1 and tx2 are producers for the tasks followed and send requests to be processded by the manager thread above.
    let tx1 = tx.clone();
    let tx2 = tx.clone();

    //Task one sends a request to set a value
    let tsk_1 = tokio::spawn(async move {
        let (t_tx, t_rx) = oneshot::channel();

        let cmd = Command::Set {
            key: "test_key".to_string(),
            val: Bytes::from("Test"),
            replier: t_tx,
        };

        tx1.send(cmd).await.expect("The msg should have been sent.");
        t_rx.await
            .expect("The test_key should be set by the manager thread.")
    });

    //Task two sends a request to retrieve the value
    let tsk_2 = tokio::spawn(async move {
        let (t_tx, t_rx) = oneshot::channel();

        let cmd = Command::Get {
            key: "test_key".to_string(),
            replier: t_tx,
        };

        tx2.send(cmd)
            .await
            .expect("The message should have been sent");

        match t_rx.await.unwrap() {
            Some(value) => {
                println!("GOT: {:?}", value);
            }
            None => {
                println!("no val found");
            }
        }
    });

    tsk_1.await.unwrap();
    tsk_2.await.unwrap();
    manager.await.unwrap();
}

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        replier: oneshot::Sender<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        replier: oneshot::Sender<()>,
    },
}
