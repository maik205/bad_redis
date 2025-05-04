use bytes::Bytes;
use mini_redis::{Connection, Frame};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
mod connection;
mod frame;
type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:6379";
    let listener = TcpListener::bind(addr).await.unwrap();

    let db: Db = Arc::new(Mutex::new(HashMap::new()));

    println!("DB listening at {}", addr);

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();
        tokio::spawn(async move {
            process(socket, Arc::clone(&db)).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, *};

    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let res = match Command::from_frame(frame).unwrap() {
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }

            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());

                Frame::Simple("OK".to_string())
            }

            _ => panic!("Unimplemented"),
        };
        connection.write_frame(&res).await.unwrap();
    }
}
