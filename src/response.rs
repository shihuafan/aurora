use std::fs::File;
use std::collections::HashMap;
use std::io::Read;

pub struct Response {
    version: String,
    status: String,
    head: HashMap<String, String>,
    text: Option<String>,
    bytes: Option<Box<[u8]>>,
    file: Option<String>
}

impl Response{
    pub fn new() -> Response{
        return Response{
            version: "HTTP/1.1".to_string(),
            status: "200 OK".to_string(),
            head: HashMap::new(),
            text: Option::None,
            bytes: Option::None,
            file: Option::None
        }
    }

    pub fn write_head(&mut self, key: &str, value: &str){
        self.head.insert(key.to_string(), value.to_string());
    }

    pub fn write_text(&mut self, body: &str){
        self.text = Option::from(body.to_string());
    }

    pub fn write_file(&mut self, path: &str){
        self.file = Option::from(path.to_string());
    }

    pub fn get_bytes(&self) -> Vec<u8>{
        let mut bytes = Vec::<u8>::new();
        self.write_status_line(&mut bytes);
        for key in self.head.keys(){
            let value = self.head.get(key);
            if value.is_some(){
                self.write_head_line(&mut bytes, key, value.unwrap());
            }
        }
        self.write_white_space_line(&mut bytes);
        self.write_body(&mut bytes);
        return bytes;
    }

    fn write_status_line(&self, bytes: &mut Vec<u8>){
        bytes.extend_from_slice(self.version.as_bytes());
        bytes.extend_from_slice(" ".as_bytes());
        bytes.extend_from_slice(self.status.as_bytes());
        bytes.extend_from_slice("\n".as_bytes());
    }

    fn write_head_line(&self, bytes: &mut Vec<u8>, key: &str, value: &str){
        bytes.extend_from_slice(key.as_bytes());
        bytes.extend_from_slice(": ".as_bytes());
        bytes.extend_from_slice(value.as_bytes());
        bytes.extend_from_slice("\n".as_bytes());
    }

    fn write_white_space_line(&self, bytes: &mut Vec<u8>){
        bytes.extend_from_slice("\n".as_bytes());
    }

    fn write_body(&self, bytes: &mut Vec<u8>){
        if self.text.is_some() {
            let text = self.text.as_ref().unwrap();
            bytes.extend_from_slice(text.as_bytes());
        }else if self.bytes.is_some() {
            bytes.extend_from_slice(self.bytes.as_ref().unwrap());
        }else if self.file.is_some() {
            let file = File::open(&self.file.as_ref().unwrap());
            if file.is_ok() {
                let mut file = file.unwrap();
                let mut buf = [0;1024];
                loop {
                    let len = file.read(&mut buf).ok();
                    if len.is_some() && len.unwrap() > 0 {
                        bytes.extend_from_slice((buf[0..len.unwrap()]).as_ref());
                    }else{
                        break();
                    }
                }
            }
        }
    }
}