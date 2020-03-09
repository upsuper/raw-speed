use crate::protocol::Mode;
use structopt::StructOpt;

mod client;
mod protocol;
mod server;
mod utils;

const DEFAULT_ADDR: &str = "0.0.0.0";
const DEFAULT_PORT: &str = "7333";

#[derive(Debug, StructOpt)]
#[structopt(name = "raw-speed", about = "Measure speed")]
enum Command {
    /// Run as server
    Server {
        /// Address to listen on
        #[structopt(short, long, default_value = DEFAULT_ADDR)]
        address: String,
        /// Port to listen on
        #[structopt(short, long, default_value = DEFAULT_PORT)]
        port: u16,
    },
    /// Run as client
    Client {
        /// Testing mode, one of 'up', 'down', and 'both'
        #[structopt(parse(try_from_str))]
        mode: Mode,
        /// Server address
        server: String,
        /// Server port
        #[structopt(default_value = DEFAULT_PORT)]
        port: u16,
    },
}

fn main() {
    match Command::from_args() {
        Command::Server { address, port } => server::run(&address, port),
        Command::Client { mode, server, port } => client::run(&server, port, mode),
    }
}
