rust
fn install_tracing() -> eyre::Result<impl Drop> {

    // You have to make sure you cast `as Box<dyn Drop>`, because Rust won't do it automatically
    Ok(Box::new(droppable) as Box<dyn Drop>)
}
