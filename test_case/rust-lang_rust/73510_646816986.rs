rust
// compiles fast
unsafe fn load_fn_570(lib: &Library) -> Option<fn(_:  _570) -> _571> { 
    Some(mem::transmute(lib.get(b"function_570")?)) 
}
unsafe fn load_fn_571(lib: &Library) -> Option<fn(_:  _571) -> _572> {
     Some(mem::transmute(lib.get(b"function_571")?)) 
}



// compiles fast
fn load_dll() -> Option<Dll> {
    // <...>
    let function_570 = load_fn_570(&lib)?;
    let function_571 = load_fn_571(&lib)?;
    // <...>
}
