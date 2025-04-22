use std::io::{BufRead, Cursor, Read};

use bytes::{Buf, Bytes, BytesMut};
use tokio::{io::AsyncReadExt, net::TcpStream};

const KB_B: usize = 1024;
const BUF_SZ_KB: usize = 4;
const EOF: usize = 0;

struct Connection {
    stream: TcpStream,
    buffer: BytesMut,
    cursor: usize,
}

impl Connection {
    pub async fn new(tcp: TcpStream) -> Connection {
        Connection {
            stream: tcp,
            buffer: BytesMut::with_capacity(BUF_SZ_KB * KB_B),
            cursor: 0,
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
        // i dont remember anything
        // i think i should write this shit from scratch
        // send help
    }

    pub async fn parse_frame(&mut self) -> Result<Option<Frame>, FrameError> {
        todo!();
    }
}

// The Redis protocol implements the transfer of data as `Frames`
enum Frame {
    Simple(String),
    Err(String),
    Integer(u64),
    Bulk(Bytes),
    Array(Vec<Frame>),
    Null,
}
type CursorBuf = &mut Cursor<&[u8]>;
impl Frame {
    pub async fn parse_buffer(cursor: CursorBuf) {}
    pub async fn check_buffer(cursor: CursorBuf) -> Result<(), FrameError> {
        match cursor.get_u8() {
            //Simple str & Err
            b'+'|
            b'-' => {
                let mut str_buf = String::new();
                cursor.read_to_string(&mut str_buf)?;
                Ok(())
            }
            //Integer
            b':' => {
                let mut str_buf = String::new();
                cursor.read_to_string(&mut str_buf)?;
                Ok(())
            }
            //Bulk Strings
            b'$' => {
                let mut str_buf = String::new();
                cursor.read_line(&mut str_buf)?;
                if (str_buf.re)                
                Ok(());
            }
            //Arrays
            b'*' => {}
            //Null
            b'_' => {}
            _ => {}
        }
    }
}

enum FrameError {
    ConnReset,
    Incomplete,
}
