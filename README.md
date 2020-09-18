```toml
[dependencies]
aurora = { path = "../aurora" }
```
```rust
extern crate aurora;

use aurora::aurora::Aurora;
use aurora::request::Request;
use aurora::response::Response;

fn main() {
    let mut a = Aurora::default();
    a.add_listener("/shihuafan", my_test);
    a.add_listener("/", default);
    a.run("127.0.0.1", "1234");
}

fn my_test(_request: &Request, _response: &mut Response){
    println!("my_test")
}

fn default(_request: &Request, _response: &mut Response){
    println!("default")
}
```
