use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::Write;

use crate::request::Request;
use crate::response::Response;
use threadpool::ThreadPool;

pub struct Aurora {
    pub listener_map: HashMap<Box<String>, Box<fn(&Request, &mut Response)>>,
    pub pool: ThreadPool
}

impl Default for Aurora{
    fn default() -> Self {
        Aurora{ listener_map: HashMap::new(), pool: ThreadPool::new(4) }
    }
}

impl Aurora{
    pub fn add_listener(&mut self, url: &str, func: fn(&Request, &mut Response)){
        self.listener_map.insert(Box::new(url.to_string()), Box::new(func));
    }

    pub fn run(&self, host: &str, port: &str){
        let mut address = String::from(host);
        address.push_str(":");
        address.push_str(port);
        println!("start listener {}", address);

        let listener = TcpListener::bind(address).unwrap();

        for stream in listener.incoming() {
            self.process_request(stream.unwrap());
        }
    }

    fn process_request(&self, mut s: TcpStream){
        let mut map = self.get_map();
        self.pool.execute(move || {
            let request = Request::new(&mut s);
            let mut response = Response::new();
            let func = match_func(&request.url, &mut map);
            if func.is_some(){
                let func = func.unwrap();
                func(&request, &mut response);
                let bytes = response.get_bytes();
                s.write(bytes.as_slice()).unwrap();
                s.flush().unwrap();
            }else{
                s.write("HTTP 404 NOT FOUND\r\n\r\n".as_bytes()).unwrap();
                s.flush().unwrap();
            }
        });
    }

    fn get_map(&self) -> HashMap<Box<String>, Box<fn(&Request, &mut Response)>>{
        let mut map = HashMap::new();
        for url in self.listener_map.keys() {
            let func = self.listener_map.get(url).unwrap();
            map.insert(url.clone(), func.clone());
        };
        return map;
    }
}

pub fn match_func<'a>(url: &'a str, map: &'a mut HashMap<Box<String>, Box<fn(&Request, &mut Response)>>)
    -> Option<&'a Box<fn(&Request, &mut Response)>>{
    let value = if map.contains_key(&url.to_string()){
        map.get(&url.to_string())
    } else {
        map.get(&"/".to_string())
    };
    return value;

}