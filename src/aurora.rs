use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::Write;

use crate::request::Request;
use crate::response::Response;
use threadpool::ThreadPool;
use std::sync::{Arc, RwLock, RwLockReadGuard};

pub struct Aurora {
    listener_map: Arc<RwLock<HashMap<Box<String>, Box<fn(&Request, &mut Response)>>>>,
    pool: ThreadPool,
}

impl Default for Aurora{
    fn default() -> Self {
        Aurora{ listener_map: Arc::new(RwLock::new(HashMap::new())),
            pool: ThreadPool::new(4),}
    }
}

impl Aurora{
    pub fn add_listener(&mut self, url: &str, func: fn(&Request, &mut Response)){
        let map = self.listener_map.clone();
        let mut a = map.write().unwrap();
        a.insert(Box::new(url.to_string()), Box::new(func));
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

    fn process_request(&self, mut stream: TcpStream){
        let arc_map = self.listener_map.clone();
        self.pool.execute(move || {
            let request = Request::new(&mut stream);
            let mut response = Response::new();
            let map = arc_map.read().unwrap();
            let func = match_func(&request.url, &map);
            if func.is_some(){
                let func = func.unwrap();
                func(&request, &mut response);
                let bytes = response.get_bytes();
                stream.write(bytes.as_slice()).unwrap();
                stream.flush().unwrap();
            }else{
                stream.write("HTTP 404 NOT FOUND\r\n\r\n".as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        });
    }

}

pub fn match_func<'a>(url: &'a str, map: &'a RwLockReadGuard<HashMap<Box<String>, Box<fn(&Request, &mut Response)>>>)
    -> Option<&'a Box<fn(&Request, &mut Response)>>{
    let value = if map.contains_key(&url.to_string()){
        map.get(&url.to_string())
    } else {
        map.get(&"/".to_string())
    };
    return value;

}