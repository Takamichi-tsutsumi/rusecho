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

    // let addr = "127.0.0.1:8080".parse().unwrap();
    // let listner: TcpListener = TcpListener::bind(&addr).expect("unable to bind TCP Listner");
    // println!("Listening on port 8080");
    //
    // let server = listner
    //     .incoming()
    //     .map_err(|e| eprintln!("accept failed = {:?}", e))
    //     .for_each(|sock| {
    //         let (reader, writer) = sock.split();
    //
    //         let bytes_copied = copy(reader, writer);
    //         let handle_conn = bytes_copied
    //             .map(|amt| println!("wrote {:?}", amt))
    //             .map_err(|err| eprintln!("IO error {:?}", err));
    //
    //         tokio::spawn(handle_conn)
    //     });
    //
    // tokio::run(server);
}
