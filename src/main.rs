use crate::protocol::Mode;
use clap::Parser;
use std::str::FromStr;

mod client;
mod protocol;
mod server;
mod utils;

const DEFAULT_ADDR: &str = "0.0.0.0";
const DEFAULT_PORT: &str = "7333";

#[derive(Debug, Parser)]
#[command(name = "raw-speed", about = "Measure speed")]
enum Command {
    /// Run as server
    Server {
        /// Address to listen on
        #[arg(short, long, default_value = DEFAULT_ADDR)]
        address: String,
        /// Port to listen on
        #[arg(short, long, default_value = DEFAULT_PORT)]
        port: u16,
    },
    /// Run as client
    Client {
        /// Testing mode, one of 'up', 'down', and 'both'
        #[arg(value_parser = Mode::from_str)]
        mode: Mode,
        /// Server address
        server: String,
        /// Server port
        #[arg(default_value = DEFAULT_PORT)]
        port: u16,
    },
}

fn main() {
    match Command::parse() {
        Command::Server { address, port } => server::run(&address, port),
        Command::Client { mode, server, port } => client::run(&server, port, mode),
    }
}
