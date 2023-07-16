rust
#![feature(async_await)]

async fn f() -> Result<bool, ()> {
    async {
        Err(())?
    }.await;
    
    Ok(true)
}
