rust
lazy_static! { static ref V: Mutex<Vec<u8>> = Mutex::new(vec![]); }

#[should_panic] #[test] fn foo() { unsafe {
    { let mut v = V.lock().unwrap(); v.reserve(42); v.set_len(42); }
    panic!();
}}
#[test] fn bar() {
    let v = V.lock().unwrap(); let v: &Vec<u8> = &*v;
    for i in v {    dbg!(*i); }
}
