use std::{net::TcpListener, thread::spawn};

use tungstenite::accept;

fn main() {
    let server = TcpListener::bind("127.0.0.2:9001").unwrap();
    for stream in server.incoming() {
        spawn (move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read_message();
                match msg {
                    Ok(msg) => {
                        if msg.is_binary() || msg.is_text() {
                            websocket.write_message(msg).unwrap();
                        }
                    }
                    Err(err) => {
                        println!("{:?}", err);
                        break;
                    }
                }
            }
        });
    }
}
