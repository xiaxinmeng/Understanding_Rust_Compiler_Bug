rust
async fn proxy(buf: &[u8]) -> Result<Vec<u8>> {
    let tv = unsafe { TIMEOUT };
    let dur = Duration::from_millis(unsafe { TIMEOUT });
    timeout(Duration::from_millis(unsafe { TIMEOUT }),
            async { Result::Ok(vec![0u8]) })
        .await?
        ;

    Result::Ok(vec![0u8])
}
