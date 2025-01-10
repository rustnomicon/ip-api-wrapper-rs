# Simple API Wrapper IP-API.com

Just install
```
cargo install ip-api-wrapper
```
And use
```rust
#[tokio::main]
async fn main() {
    let client = Client::new();
    let ip = "8.8.8.8";
    let info_full = IPInfo::get_info(&client, ip).await.unwrap();
    println!("{:#?}", info_full.as_field);
}
```