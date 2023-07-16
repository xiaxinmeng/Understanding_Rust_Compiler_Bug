rust
#![feature(async_fn_in_trait)]

trait AsyncTrait {
    async fn foo(&self) -> Result<(), ()>;
}

struct AsyncStruct;

impl AsyncTrait for AsyncStruct {
    async fn foo(&self) -> Result<(), ()> {
        Ok(())
    }
}

async fn run_async_workflow(a: &impl AsyncTrait) -> Result<(), ()> {
    a.foo().await
}

fn main() {
    let _ = async {
        let obj = AsyncStruct;
        let _ = run_async_workflow(&obj).await;
    };
}
