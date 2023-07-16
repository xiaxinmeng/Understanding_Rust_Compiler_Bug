 rust
let args = args().map(|arg| CString::new(arg).unwrap()).collect::<Vec<_>>();
let arg_ptrs = args.iter().map(|arg| arg.as_ptr()).collect::<Vec<_>>();
unsafe {
    printargs(arg_ptrs.len(), arg_ptrs.as_ptr());
}
