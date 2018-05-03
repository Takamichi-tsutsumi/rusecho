extern crate tokio;

use tokio::prelude::*;
use tokio::io::copy;
use tokio::net::TcpListener;

fn main() {
    let addr = "127.0.0.1:3333".parse().unwrap();
    let listner: TcpListener = TcpListener::bind(&addr).expect("unable to bind TCP Listner");

    let server = listner
        .incoming()
        .map_err(|e| eprintln!("accept failed = {:?}", e))
        .for_each(|sock| {
            let (reader, writer) = sock.split();

            let bytes_copied = copy(reader, writer);
            let handle_conn = bytes_copied
                .map(|amt| println!("wrote {:?}", amt))
                .map_err(|err| eprintln!("IO error {:?}", err));

            tokio::spawn(handle_conn)
        });

    tokio::run(server);
}
