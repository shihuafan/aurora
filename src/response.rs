
pub struct Response {
    pub version: String,
    pub status: String,
    pub body: Option<String>,
}

impl Response{
    pub fn new() -> Response{
        return Response{
            version: "HTTP/1.1".to_string(),
            status: "200 OK".to_string(),
            body: Some(String::from("shihuafan")),
        }
    }

    pub fn get_bytes(&self) -> Vec<u8>{
        let mut bytes = Vec::<u8>::new();
        self.write_status_line(&mut bytes);
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
        if self.body != Option::None {
            let body = self.body.as_ref().unwrap();
            bytes.extend_from_slice(body.as_bytes());
        }
    }
}