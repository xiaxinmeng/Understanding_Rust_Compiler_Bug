rust
match void_ref_result {
    Err(e) => ... ,
    Ok(&r) => match r { },
}
