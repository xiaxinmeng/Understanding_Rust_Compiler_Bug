rust
async fn run_with_ctx<Ret, Fn, Fut>(a: Fn) -> Ret
where
    for<'a> {
    	Fn: FnOnce(&'a mut i32) -> Fut,
    	Fut: Future<Output = Ret> + 'a
    }
{
    a(&mut 0i32).await
}
