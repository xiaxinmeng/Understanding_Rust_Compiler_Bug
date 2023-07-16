
async fn previously() -> Result<_, lib::Error> {
    let _ = await get_result()?;
}

async fn with_recovery() -> Result<_, lib::Error> {
    // Does `or_recover` return a future or not? Suddenly very important but not visible.
    let _ = await get_result().unwrap_or_else(or_recover);
    // If `or_recover` is sync, this should still work as a pattern of replacing `?` imho.
    // But we also want `or_recover` returning a future to work, as a combinator for futures?
   
    // Resolving sync like this just feel like wrong precedence in a number of ways
    // Also, conflicts with `Result of future` depending on choice.
    let _ = await get_result()?.unwrap_or_else(or_recover);
}
