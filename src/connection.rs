use std::io::{BufRead, Cursor, Read};

use crate::frame::{Frame, FrameError};
use bytes::{Buf, Bytes, BytesMut};
use tokio::{io::AsyncReadExt, net::TcpStream};
const KB_B: usize = 1024;
const BUF_SZ_KB: usize = 4;
const EOF: usize = 0;

struct Connection {
    stream: TcpStream,
    buffer: BytesMut,
}

impl Connection {
    pub async fn new(tcp: TcpStream) -> Connection {
        Connection {
            stream: tcp,
            buffer: BytesMut::with_capacity(BUF_SZ_KB * KB_B),
        }
    }

    pub async fn read_frame(&mut self) -> Result<Option<Frame>, FrameError> {
        loop {
            if let Some(frame) = self.parse_frame().await? {
                return Ok(Some(frame));
            }

            if self.stream.read_buf(&mut self.buffer).await.unwrap() == EOF {
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err(FrameError::ConnReset);
                }
            }
        }
    }

    pub async fn write_frame(&mut self, frame: Frame) {
        todo!();
        // let (_, writer) = self.stream.split();
        // let tsk = tokio::spawn(async move {});
    }

    pub async fn parse_frame(&mut self) -> Result<Option<Frame>, FrameError> {
        todo!();
    }
}
