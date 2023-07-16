rust
#[tokio::main]
pub async fn main() {
    // this call causes the Future from `callee()`
    // to be materialized on stack and memcpy-moved
    // to the future of this function
    callee().await;
}

#[inline(never)] // prevent inlining creating the generator
pub async fn callee() {
    let mut data = [0_u32; 128];
    tokio::task::yield_now().await;
    println!("addr {:p}", &mut data);
}
