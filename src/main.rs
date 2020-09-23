mod aurora;
mod request;
mod response;

use self::aurora::Aurora;
use crate::request::Request;
use crate::response::Response;

fn main() {
    let mut aurora = Aurora::default();
    aurora.add_listener("/shihuafan", my_test);
    aurora.add_listener("/", default);
    aurora.add_listener("/static", get_static);
    aurora.run("127.0.0.1", "9264");
}

fn my_test(_request: &Request, response: &mut Response){
    println!("==my_test==");
    response.write_text("");
    response.write_head("name", "wss");
}

fn default(_request: &Request, response: &mut Response){
    println!("==default==");
    response.write_file("/Users/shihuafan/../shihuafan/desktop/rust_image.png")
}

fn get_static(request: &Request, response: &mut Response){
    let path = request.query.get(&"path".to_string());
    if path.is_some() {
        response.write_file(path.unwrap());
        println!("{}",path.unwrap());
    }
}