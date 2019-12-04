use std::fmt;
extern crate posix_mq;
extern crate posixmq;
#[macro_use]
extern crate rustler;

use posix_mq::*;
use posixmq::PosixMq;
use rustler::error::Error;
use rustler::*;
mod atoms {
    rustler_atoms! {
        atom ok;
    }
}

#[derive(Debug)]
struct MyError {
    details: String,
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}

rustler_export_nifs!("posix_queue", [("send", 2, send),], Some(on_load));

#[no_mangle]
fn on_load(_env: Env, _load_info: Term) -> bool {
    true
}

pub fn send<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let name: String = args[0].decode()?;
    let msg: String = args[1].decode()?;
    let qname = match Name::new(name) {
        Ok(x) => x,
        Err(e) => return Err(rustler::error::Error::BadArg),
    };
    match send_internal(qname, msg) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e.to_string());
            return Err(rustler::error::Error::RaiseAtom("Queue open failed"));
        }
    }
    Ok(atoms::ok().encode(env))
}

fn send_internal<'a>(name: Name, msg: String) -> Result<(), Box<dyn std::error::Error + 'static>> {
    match Queue::open_or_create(name) {
        Ok(queue) => {
            match queue.send(&Message {
                data: msg.as_bytes().to_vec(),
                priority: 0,
            }) {
                Ok(_) => Ok(()),
                Err(e) => {
                    return Err(Box::new(MyError::new(&format!(
                        "Sending message failed {}",
                        e.to_string()
                    ))));
                }
            }
        }
        Err(e) => {
            eprintln!("Queue creation failed: {}", e.to_string());
            return Err(Box::new(MyError::new(&format!(
                "Queue creation/open failed"
            ))));
        }
    }
}

#[test]
fn test() {
    let name = Name::new("/testqueue").unwrap();
    match send_internal(name.clone(), "foo".to_string()) {
        Ok(_) => (),
        Err(_) => panic!("Error"),
    }
    let queue = Queue::open_or_create(name).unwrap();
    let result = queue.receive().unwrap();
    let foo: String = String::from_utf8(result.data).unwrap();
    assert_eq!(foo, "foo".to_string());
}
