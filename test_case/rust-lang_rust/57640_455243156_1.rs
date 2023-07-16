
async fn with_recovery() -> Result<_, lib::Error> {
    // Also possible in 'space' delimited post-fix await route, but slightly less clear
    let _ = get_result()(await)
        // Ah, this is sync
        .unwrap_or_else(or_recover);
    // This would be future combinator.
    // let _ = get_result().unwrap_or_else(or_recover)(await);
}
