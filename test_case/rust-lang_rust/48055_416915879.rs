rust
fn new() -> dyn Debug {
    let _buf1 = [0u8; 1 << 10]; // this remains allocated "after returning" ...
    "hello world" // ... because here we actually call take()
}
fn take(arg: dyn Debug) {
    let _buf2 = [0u8; 1 << 10];
    println!("{:?}", arg);
}
fn foo() {
    take(new()); // 2 KB peak stack usage, not 1 KB
}
