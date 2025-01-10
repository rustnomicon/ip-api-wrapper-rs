# Simple API Wrapper IP-API.com

Simple code
```rust
#[tokio::main]
async fn main() {
    let client = Client::new();
    let ip = "8.8.8.8";
    let info_full = IPInfo::get_info(&client, ip).await.unwrap();
    println!("{:#?}", info_full.as_field);
}
```