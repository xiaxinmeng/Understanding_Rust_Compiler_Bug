rust
//idea here is AsyncClosure argument and return future have the same lifetime
trait AsyncClosure<'a,Argument,Output> {
        type Fut: std::future::Future<Output = Output> + 'a;
        fn call(self, arg: &'a Argument) -> Self::Fut;
}

//blanket impl
impl<'a, Fu: 'a, F,Argument: 'static,Output> AsyncClosure<'a,Argument,Output> for F
where
    F: FnOnce(&'a Argument) -> Fu,
    Fu: std::future::Future<Output = Output> + 'a,
{
    type Fut = Fu;
    fn call(self, rt: &'a Argument) -> Fu {
        self(rt)
    }
}

async fn with_async_closure< C,R>(c: C) -> R where for<'a> C: AsyncClosure<'a,u8,R> {
	let a = 3; //closure borrows this
	c.call(&a) //returned future borrows it as well
	.await
	//no longer borrowed here
}

async fn function_target(arg:&u8) {
    println!("{:?}",arg);
}

async fn amain() {
    with_async_closure(function_target); //works on bare fn
   with_async_closure( |arg| async { function_target(arg) } ); //error: implementation of `AsyncClosure` is not general enough
}

