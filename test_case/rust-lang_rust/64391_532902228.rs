rust
async fn connect() {
    let config = 666;
    connect2(&config, "".to_string()).await
}

async fn connect2(_config: &u32, _tls: String) {
    unimplemented!()
}

