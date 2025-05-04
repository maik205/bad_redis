use std::io::Cursor;

use bytes::{ Buf, Bytes };
use tokio::io::AsyncReadExt;

// The Redis protocol implements the transfer of data as `Frames`
pub enum Frame {
    Simple(String),
    Err(String),
    Integer(u64),
    Bulk(Bytes),
    Array(Vec<Frame>),
    Null,
}
pub type CursorBuf<'a> = &'a mut Cursor<&'a [u8]>;
impl Frame {
    pub async fn parse<'a>(cursor: CursorBuf<'a>) {}
    // Reads the frame from beginning to end and check if a full frame is recieved

    pub async fn check<'a>(cursor: CursorBuf<'a>) -> Result<(), FrameError> {
        // Check the start of the frame

        match cursor.get_u8() {
            //Simple str & Err
            b'+' | b'-' => {
                get_line(cursor)?;
                Ok(())
            }
            //Integer
            b':' => {
                let line = get_line(cursor)?;
                Ok(())
            }
            //Bulk Strings
            //$<len>\r\n<data>\r\n
            //null: $-1\r\n (RESP2)
            b'$' => {
                if *peek_u8(cursor)? == b'-' {
                    skip(cursor, 4);
                } else {
                }
                Ok(())
            }
            //Arrays
            b'*' => {}
            //Null
            b'_' => {}

            // Others
            _ => {}
        }
    }
}

//Utility functions for cursor manipulation

fn peek_u8<'a>(cursor: CursorBuf<'a>) -> Result<&'a u8, FrameError> {
    if cursor.has_remaining() {
        return Ok(&cursor.get_ref()[cursor.position() as usize]);
    }
    Err(FrameError::Incomplete)
}

fn get_u8<'a>(cursor: CursorBuf<'a>) -> Result<u8, FrameError> {
    if cursor.has_remaining() {
        return Ok(cursor.get_u8());
    }
    Err(FrameError::Incomplete)
}

fn get_decimal<'a>(cursor: CursorBuf<'a>) -> Result<i64, FrameError> {
    let line = get_line(cursor).unwrap();
    use atoi::atoi;
    atoi::<i64>(line).ok_or_else(|| FrameError::InvalidFrame)
}

/// Finds and returns the next valid byte slice from given cursor.
fn get_line<'a>(cursor: CursorBuf<'a>) -> Result<&'a [u8], FrameError> {
    // Referencing the Mini-Redis crate implementation for this one...
    // The crate's implementation starts from the cursor's current position and scans
    // until the end until it finds a new line, then returns the line found.
    let begin = cursor.position() as usize;
    // Might be better to scan the end first for CRLF characters for better validation
    // On second thought its not needed since the function would return incomplete
    // irrespective of prechecks
    // stop thinking about microoptimizations lol
    let end = cursor.get_ref().len() - 1;

    for i in begin..end {
        if cursor.get_ref()[i] == b'\r' && cursor.get_ref()[i + 1] == b'\n' {
            cursor.set_position((i + 2) as u64);
            return Ok(&cursor.get_ref()[begin..i]);
        }
    }
    Err(FrameError::Incomplete)
}

fn skip(src: CursorBuf, n: usize) -> Result<(), FrameError> {
    if src.remaining() < n {
        return Err(FrameError::Incomplete);
    }

    src.advance(n);
    Ok(())
}

#[derive(Debug)]
pub enum FrameError {
    ConnReset,
    Incomplete,
    InvalidFrame,
}
