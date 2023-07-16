
async fn compile_fail_without_return() -> Result<i32, i32> {
    Result::<i32, i32>::Ok(20i32)?;
    //Result::<i32, i32>::Ok(20i32) If you add me, stable gives a sane message.
}

fn main() {}
