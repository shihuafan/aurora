mod aurora;
mod request;
mod response;

use self::aurora::Aurora;
use crate::request::Request;
use crate::response::Response;

fn main() {
    let mut aurora = Aurora::default();
    aurora.add_router("/shihuafan" , my_test);
    aurora.add_router("/" , default);
    aurora.run("127.0.0.1", "1234");
}

fn my_test(_request: &Request, _response: &mut Response){
    println!("my_test")
}

fn default(_request: &Request, _response: &mut Response){
    println!("default")
}