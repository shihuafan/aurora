use std::collections::HashMap;
use std::net::TcpListener;
use std::io::{Write, Read};
use std::time::Duration;

use crate::request::Request;
use crate::response::Response;
use crate::thread_pool::ThreadPool;

pub struct Aurora {
    pub routers: HashMap<Box<String>, Box<fn(&Request, &mut Response)>>,
}

impl Default for Aurora{
    fn default() -> Self {
        Aurora{routers: HashMap::new() }
    }
}

impl Aurora{
    pub fn add_router(&mut self, url: &str, func: fn(&Request, &mut Response)){
        self.routers.insert(Box::new(url.to_string()), Box::new(func));
    }

    pub fn get_urls(&mut self) -> Vec<String>{
        let mut urls = Vec::<String>::new();
        for url in self.routers.keys() {
            urls.push(url.to_string());
        };
        return urls;
    }

    pub fn run(&self, host: &str, port: &str){
        let mut address = String::from(host);
        address.push_str(":");
        address.push_str(port);
        println!("start listener {}", address);
        let listener = TcpListener::bind(address).unwrap();
        let thread_pool = ThreadPool{};
        for stream in listener.incoming() {
            let mut s = stream.unwrap();
            let request = Request::new(&mut s);
            let mut response = Response::new();
            for url in self.routers.keys() {
                let func = self.routers.get(url).unwrap();
                func(&request, &mut response);
                let bytes = response.get_bytes();
                s.write(bytes.as_slice()).unwrap();
                s.flush().unwrap();
                break;
            };
        }
    }
}
