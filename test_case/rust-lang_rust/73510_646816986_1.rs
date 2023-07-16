rust
// compiles slow
fn load_dll() -> Option<Dll> {
    // <...>
    let function_570: fn(_:  _570) -> _571 = mem::transmute(lib.get(b"function_570")?);
    let function_571: fn(_:  _571) -> _572 = mem::transmute(lib.get(b"function_571")?);
    // <...>
}
