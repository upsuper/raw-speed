use bitflags::bitflags;
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

macro_rules! buf_size {
    () => {
        1024 * 1024
    };
}
macro_rules! impl_send {
    ($socket:ident : $n:ident => $handler:block) => {
        let buf = [0u8; buf_size!()];
        loop {
            match $socket.write(&buf) {
                Ok(0) => break,
                Ok($n) => $handler,
                Err(e) => {
                    if e.kind() != ErrorKind::Interrupted {
                        panic!("Failed to send data: {:?}", e);
                    }
                }
            }
        }
    };
}
macro_rules! impl_recv {
    ($socket:ident : $n:ident => $handler:block) => {
        let mut buf = [0u8; buf_size!()];
        loop {
            match $socket.read(&mut buf) {
                Ok(0) => break,
                Ok($n) => $handler,
                Err(e) => {
                    if e.kind() != ErrorKind::Interrupted {
                        panic!("Failed to receive data: {:?}", e);
                    }
                }
            }
        }
    };
}
