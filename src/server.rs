use crate::protocol::{self, receive_indefinitely, send_indefinitely, Mode};
use crate::utils;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

pub fn run(addr: &str, port: u16) {
    let listener = TcpListener::bind((addr, port)).expect("Failed to start listening");
    let local_addr = listener
        .local_addr()
        .expect("Failed to get the local address?");
    println!("Start listening on {}...", local_addr);

    loop {
        match listener.accept() {
            Err(e) => println!("Couldn't get client: {:?}", e),
            Ok((socket, addr)) => {
                utils::create_thread(format!("conn-{}", addr), move || handle_connection(socket))
                    .unwrap_or_else(|e| {
                        println!(
                            "Failed to handle connection \
                     from {}: {:?}",
                            addr, e
                        );
                    })
            }
        }
    }
}

fn handle_connection(mut socket: TcpStream) {
    let addr = socket.peer_addr().expect("No peer address?");
    println!("Get connection from {}...", addr);

    let mut magic = [0u8; 4];
    socket
        .read_exact(&mut magic)
        .expect("Failed to read magic number");
    if magic != *protocol::MAGIC_NUMBER {
        println!("Broken connection from {}...", addr);
        return;
    }

    let mut mode = [0u8; 1];
    socket.read_exact(&mut mode).expect("Failed to read mode");
    let mode = match protocol::Mode::from_bits(mode[0]) {
        Some(mode) => mode,
        None => {
            println!("Unknown working mode from {}...", addr);
            return;
        }
    };

    if mode == Mode::UP {
        handle_upstream(socket);
    } else if mode == Mode::DOWN {
        handle_downstream(socket);
    } else if mode == Mode::UP | Mode::DOWN {
        let socket2 = socket.try_clone().expect("Failed to duplicate socket");
        utils::create_thread(format!("up-{}", addr), move || handle_upstream(socket2))
            .expect("Failed to handle upstream connection");
        handle_downstream(socket);
    }
}

/// Receive upstream data from the client.
fn handle_upstream(socket: TcpStream) {
    receive_indefinitely(socket, |_| {});
}

/// Send downstream data to the client.
fn handle_downstream(socket: TcpStream) {
    send_indefinitely(socket, |_| {});
}
