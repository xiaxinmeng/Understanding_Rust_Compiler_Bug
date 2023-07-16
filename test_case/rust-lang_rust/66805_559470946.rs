rust
pub fn func() {}

fn main() {
    let s = async_stream::stream! {
        Ok(0_i32)?;
        yield func();
    };  
}
