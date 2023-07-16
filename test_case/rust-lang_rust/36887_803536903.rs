rust
fn some_fn<T: MyTrait = DefaultMyTrait>(text: &str, another_param: usize) -> Result<(), ()>{
    T::check_text(text);
    // some more operations
    Ok(())
}
