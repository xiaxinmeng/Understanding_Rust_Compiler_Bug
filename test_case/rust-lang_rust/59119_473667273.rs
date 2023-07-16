rust
async fn my_future(ctx: Context) -> () {
    println!("Context field: {}", ctx.field);
    ()
}
