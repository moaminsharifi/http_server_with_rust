# HTTP/1.1 Web server with rust and with serving html and assets feature with single thread architecture

## 1. install rust on your machine 
best document for install is own [Rust documention](https://www.rust-lang.org/tools/install)

for linux users download and run rust bash file
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
then just add this line to your shell. for bash:
```
echo 'source $HOME/.cargo/env' >> ~/.bashrc
```

## 2. Run

just use it!
```
cargo run
```

## 3. Add new route

open `src/website_handler.rs` and go to implementation of Handler for WebsiteHandler

```rust
impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request:&crate::http::request::Request) -> crate::http::Response {
        match request.method() {
            Method::GET => match request.path() {
            "/"=> Response::new(StatusCode::Ok , self.read_file("index.html")),
            "/test"=> Response::new(StatusCode::Ok , self.read_file("test.html")),
            path => match self.read_file(path) {
                Some(contents) => Response::new(StatusCode::Ok , Some(contents)),
                None => Response::new(StatusCode::NotFound , Some("<h1>Page not found</h1>".to_string()))
            }
            }
            _ =>  Response::new(StatusCode::NotFound , None)
        }
        
    }
    
}
```
add new route after `match request.method() {` line
like:
1- add to get method like:
```rust
Method::GET => match request.path() {
            "/"=> Response::new(StatusCode::Ok , self.read_file("index.html")),
            "/404"=> Response::new(StatusCode::NotFound , self.read_file("404.html")),
            ...

```
or add new method with:
```rust
Method::POST => match request.path() {
            "/login"=> {
                // code and code and more code
                Response::new(StatusCode::Ok , self.read_file("login.html"))
                },
            // consider at least add default case with underscore `_`
            _ =>  Response::new(StatusCode::NotFound , None)
}


```