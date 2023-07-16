rust
async fn foo() -> Result<(), String> {
    Ok(())
}

fn convert_result<T, E>(r: Result<T, E>) -> Option<T> {
    None
}

fn main() -> Option<()> {
    convert_result(foo())
}
