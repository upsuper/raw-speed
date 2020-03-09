use bitflags::bitflags;
use std::io::{ErrorKind, Read, Write};
use std::net::TcpStream;
use std::str::FromStr;

pub const MAGIC_NUMBER: &[u8; 4] = b"SPED";

bitflags! {
    pub struct Mode: u8 {
        /// Testing upstream speed from client to server.
        const UP = 1 << 0;
        /// Testing downstream speed from server to client.
        const DOWN = 1 << 1;
    }
}

impl FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "up" {
            Ok(Mode::UP)
        } else if s == "down" {
            Ok(Mode::DOWN)
        } else if s == "both" {
            Ok(Mode::UP | Mode::DOWN)
        } else {
            Err(s.into())
        }
    }
}

const BUF_SIZE: usize = 1024 * 1024;

pub fn send_indefinitely(mut socket: TcpStream, progress: impl Fn(usize)) {
    let buf = [0u8; BUF_SIZE];
    loop {
        match socket.write(&buf) {
            Ok(0) => break,
            Ok(n) => progress(n),
            Err(e) => {
                if e.kind() != ErrorKind::Interrupted {
                    panic!("Failed to send data: {:?}", e);
                }
            }
        }
    }
}

pub fn receive_indefinitely(mut socket: TcpStream, progress: impl Fn(usize)) {
    let mut buf = [0u8; BUF_SIZE];
    loop {
        match socket.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => progress(n),
            Err(e) => {
                if e.kind() != ErrorKind::Interrupted {
                    panic!("Failed to receive data: {:?}", e);
                }
            }
        }
    }
}
