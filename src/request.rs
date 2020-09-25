use std::net::TcpStream;
use std::io::Read;
use std::collections::HashMap;

pub struct Request {
    pub method: String,
    pub url: String,
    pub version: String,
    pub head: HashMap<Box<String>, Box<String>>,
    pub query: HashMap<Box<String>, Box<String>>,
}

impl Request{
    pub fn new(stream: &mut TcpStream) -> Option<Request>{
        let mut string_buffer = String::new();
        loop{
            let mut buffer = [0; 1024];
            let len = stream.read(&mut buffer).unwrap();
            string_buffer.push_str(&String::from_utf8_lossy(&buffer[0..len]));
            if len < 1024 { break }
        }

        let mut lines = string_buffer.lines();

        let status_line = lines.next();
        if verify_status_line(&status_line) == false{
            return Option::None
        }

        let mut request = Request::get_request_by_head(&status_line.unwrap());

        loop {
            let line = lines.next();
            if line == Option::None || line.unwrap().len() == 0 { break };
            let index = line.unwrap().find(": ");
            if index != Option::None {
                let key = &line.unwrap()[0..index.unwrap()];
                let value = &line.unwrap()[(index.unwrap()+1)..];
                request.head.insert(Box::new(key.to_string()),
                                     Box::new(value.to_string()));
            };
        }

        // let content_length = request.head.get(&"Content-Length".to_string());
        // let content_type = request.head.get(&"Content-Type".to_string());
        loop {
            let line = lines.next();
            if line == Option::None{ break };
        }

        return Some(request);
    }

    fn get_request_by_head(head: &str) -> Request{
        let mut intertor = head.split_whitespace();
        let method = intertor.next().unwrap();
        let url_wite_query = intertor.next().unwrap();
        let version = intertor.next().unwrap();

        let url_index = url_wite_query.find('?');
        let url = if url_index != Option::None {
            &url_wite_query[0..url_index.unwrap()]
        } else {
            url_wite_query
        };

        let mut request = Request{
            method: method.to_string(),
            url: url.to_string(),
            version: version.to_string(),
            head: HashMap::new(),
            query: HashMap::new(),
        };

        if url_index != Option::None {
            let query = &url_wite_query[(url_index.unwrap()+1)..];
            let mut item_intertor = query.split('&');
            loop {
                let item = item_intertor.next();
                if item == Option::None { break };
                let ket_value = item.unwrap();
                let index = item.unwrap().find('=');
                if index != Option::None {
                    let key = &ket_value[0..index.unwrap()];
                    let value = &ket_value[(index.unwrap()+1)..];
                    request.query.insert(Box::new(key.to_string()),
                                         Box::new(value.to_string()));
                };
            };
        };
        return request;
    }
}

fn verify_status_line(status_line: &Option<&str>) -> bool{
    if status_line.is_none() {  return false };
    let status_line = status_line.unwrap();
    if status_line.contains(r"HTTP") {
        return true;
    }else{
        return false;
    }
}