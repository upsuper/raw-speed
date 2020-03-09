use clap::clap_app;

#[macro_use]
mod protocol;

mod client;
mod server;
mod utils;

const DEFAULT_ADDR: &str = "0.0.0.0";
const DEFAULT_PORT: u16 = 7333;

fn main() {
    let matches = clap_app!(rawspeed =>
        (version: "0.1")
        (author: "Xidorn Quan <me@upsuper.org>")
        (about: "Measure speed")
        (@subcommand server =>
            (about: "Run as server")
            (@arg address: -a --address +takes_value
             "Address to listen on, 0.0.0.0 by default")
            (@arg port: -p --port +takes_value
             "Port to listen on, 7333 by default")
        )
        (@subcommand client =>
            (about: "Run as client")
            (@arg mode: +required
             "Testing mode, one of 'up', 'down', and 'both'")
            (@arg server: +required "Server address")
            (@arg port: "Server port, 7333 by default")
        )
    )
    .get_matches();

    if let Some(server) = matches.subcommand_matches("server") {
        let addr = server.value_of("address").unwrap_or(DEFAULT_ADDR);
        let port = server
            .value_of("port")
            .map(|p| p.parse().expect("Invalid port number"))
            .unwrap_or(DEFAULT_PORT);
        server::run(addr, port);
    } else if let Some(client) = matches.subcommand_matches("client") {
        let mode = client
            .value_of("mode")
            .unwrap()
            .parse()
            .expect("Invalid mode");
        let addr = client.value_of("server").unwrap();
        let port = client
            .value_of("port")
            .map(|p| p.parse().expect("Invalid port number"))
            .unwrap_or(DEFAULT_PORT);
        client::run(addr, port, mode);
    } else {
        panic!("Must be run as server or client.");
    }
}
