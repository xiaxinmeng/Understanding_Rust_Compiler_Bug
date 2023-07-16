rust
iter.try_for_each(|r| {
    let _ = r?;
    Ok(())
})?;
