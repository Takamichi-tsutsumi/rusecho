extern crate tokio;

use tokio::prelude::*;
use tokio::io;
use tokio::net::TcpListener;

fn main() {
    let addr = "127.0.0.1:12345".parse().unwrap();
    let tcp = TcpListener::bind(&addr).unwrap();

    let server = tcp.incoming()
        .for_each(|tcp| {
            let (reader, writer) = tcp.split();

            let conn = io::copy(reader, writer)
                .map(|(n, _, _)| println!("wrote {} bytes", n))
                .map_err(|err| println!("IO error {:?}", err));

            tokio::spawn(conn);
            Ok(())
        })
        .map_err(|err| {
                     println!("server error {:?}", err);
                 });

    tokio::run(server);
}
