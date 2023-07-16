 rust
static mut foo: i32 = 0;

unsafe fn bar(flag: bool) {
    if flag {
        foo = 1;
    }
    println!("{}", foo);
}
