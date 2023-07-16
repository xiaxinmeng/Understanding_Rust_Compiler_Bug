 rust
use std::io::{IoError, InvalidInput};
use std::io::net::tcp::TcpStream;

enum Packet {JsonPacket(u8, str)}

impl Packet {
    fn read_json(stream: &mut TcpStream) {
        Ok(0u).and_then(|size| {
            stream.read_byte().and_then(|command| {
                stream.read_exact(size - 1).and_then(|data| {
                    match std::str::from_utf8(data.as_slice()) {
                        Some(&json) => Ok(JsonPacket(command, json)),
                        None => Err(IoError {
                            kind: InvalidInput,
                            desc: "hi",
                            detail: None
                        })
                    }
                })
            })
        });
    }
}

fn main() {}
