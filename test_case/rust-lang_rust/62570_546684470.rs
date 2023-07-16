
async fn an_async_function() -> u32 {
    let x: Option<u32> = None;
    x?; //~ ERROR the `?` operator
    22
}
