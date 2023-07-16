rust
x.try() match {
    Ok(c) => c,
   Err(e) => throw e // throw here is just a placeholder for either returning or breaking out of a try block
}
