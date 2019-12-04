use std::{env, fmt};
extern crate posix_mq;
extern crate posixmq;
#[macro_use]
extern crate rustler;
extern crate serde_json;
use posix_mq::*;
use serde_json::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let qname: String = args[1].clone();
    let name = Name::new(qname).unwrap();
    let queue = Queue::open_or_create(name).unwrap();
    loop {
        match queue.receive() {
            Ok(msg) => {
                let json: serde_json::Value =
                    serde_json::from_str(&String::from_utf8(msg.data).unwrap()).unwrap();
                println!("{}", json.to_string());
            }
            Err(e) => panic!("{}", e.to_string()),
        }
    }
}
