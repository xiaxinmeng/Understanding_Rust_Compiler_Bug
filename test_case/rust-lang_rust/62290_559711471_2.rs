rust
async fn a() -> Result<(), reqwest::Error> {Ok(())}
async fn b() -> Result<(), std::io::Error> {Ok(())}

fn c() {
    async {
        a().await?;
        b().await?;
        Ok::<(), impl std::error::Error>(())
    };
}
