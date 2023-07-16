
async fn an_async_block() -> u32 {
    async {
        let x: Option<u32> = None;
        x?; //~ ERROR the `?` operator
        22
    }.await
}
