use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::Write;

use crate::request::Request;
use crate::response::Response;
use threadpool::ThreadPool;
use std::sync::{Arc, RwLock, RwLockReadGuard};

pub struct Aurora {
    listener_map: HashMap<Box<String>, Box<fn(&Request, &mut Response)>>,
    pool: ThreadPool,
}

impl Default for Aurora{
    fn default() -> Self {
        Aurora{ listener_map: HashMap::new(),
            pool: ThreadPool::new(4),}
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

        let map = self.get_map();
        let arc_map = Arc::new(RwLock::new(map));
        for stream in listener.incoming() {
            self.process_request(arc_map.clone(), stream.unwrap());
        }
    }

    fn process_request(&self, arc_map: Arc<RwLock<HashMap<Box<String>, Box<fn(&Request, &mut Response)>>>>, mut s: TcpStream){
        self.pool.execute(move || {
            let request = Request::new(&mut s);
            let mut response = Response::new();
            let arc_map = arc_map.read().unwrap();
            let func = match_func(&request.url, &arc_map);
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

pub fn match_func<'a>(url: &'a str, map: &'a RwLockReadGuard<HashMap<Box<String>, Box<fn(&Request, &mut Response)>>>)
    -> Option<&'a Box<fn(&Request, &mut Response)>>{
    let value = if map.contains_key(&url.to_string()){
        map.get(&url.to_string())
    } else {
        map.get(&"/".to_string())
    };
    return value;

}