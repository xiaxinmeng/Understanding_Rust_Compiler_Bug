rust
async fn robinhood_task() {
    let robinhood = Robinhood::new();
    let token = String::from("fake");
    let instrument_ids = vec![String::from("fake")];
    robinhood.get_options_market_data(token, instrument_ids).await;
}
