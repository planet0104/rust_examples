extern crate websocket;

use std::thread;
use websocket::OwnedMessage;
use websocket::sync::Server;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate serde;

use std::time::{Duration, Instant};

use bincode::{serialize, deserialize};

//精灵信息: Vec<id,x,y,res_id>
#[derive(Serialize, Deserialize, Debug)]
pub struct SData{
    pub id: u32,
    pub x: u16,
    pub y: u16,
    pub res: u16,
	pub child: Child
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Child{
    pub name: String,
}

pub fn duration_to_milis(duration: &Duration) -> f64{
    duration.as_secs() as f64 * 1000.0 + duration.subsec_nanos() as f64 / 1_000_000.0
}

fn main() {
	let server = Server::bind("127.0.0.1:8080").unwrap();

	for request in server.filter_map(Result::ok) {
		thread::spawn(move || {
			let mut client = request.accept().unwrap();

			let ip = client.peer_addr().unwrap();

			println!("Connection from {}", ip);

            let mut sprites = vec![];
            sprites.push(SData{
                x:100,y:100,id:100,res:255,
				child:Child{name: String::from("呵呵")}
            });
            sprites.push(SData{
                x:120,y:100,id:100,res:255,
				child:Child{name: String::from("abcd")}
            });
			sprites.push(SData{
                x:120,y:100,id:120,res:255,
				child:Child{name: String::from("eebb")}
            });
			sprites.push(SData{
                x:120,y:190,id:120,res:255,
				child:Child{name: String::from("855")}
            });
			for i in 0..100{
				sprites.push(SData{
					x:120,y:i/2,id:120+i as u32,res:i/2,
					child:Child{name: String::from("855")}
				});
			}

			//println!("编码之前:{:?}", sprites);

            let encoded: Vec<u8> = serialize(&sprites).unwrap();

            println!("编码之后长度:{}", encoded.len());
			let message = OwnedMessage::Binary(encoded);
			client.send_message(&message).unwrap();

			client.send_message(&OwnedMessage::Text(String::from("测试"))).unwrap();

			let (mut receiver, mut sender) = client.split().unwrap();

			for message in receiver.incoming_messages() {
				let message = message.unwrap();

				match message {
					OwnedMessage::Close(_) => {
						let message = OwnedMessage::Close(None);
						sender.send_message(&message).unwrap();
						println!("Client {} disconnected", ip);
						return;
					}
					OwnedMessage::Ping(ping) => {
						let message = OwnedMessage::Pong(ping);
						sender.send_message(&message).unwrap();
					}
					_ => sender.send_message(&message).unwrap(),
				}
			}
		});
	}
}