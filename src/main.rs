mod aurora;
mod request;
mod response;
mod thread_pool;

use self::aurora::Aurora;
use std::net::TcpStream;
use std::io::{Read, Write};
use crate::request::Request;
use crate::response::Response;
use std::process::Command;

fn main() {
    let mut aurora = Aurora::default();
    aurora.add_router("/my_test" , my_test);
    aurora.run("127.0.0.1", "1234");
}

fn my_test(request: &Request, response: &mut Response){
    println!("{}", request);
    println!("shihaufan+++++")
}
