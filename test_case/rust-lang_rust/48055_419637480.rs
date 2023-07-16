rust
fn new() -> dyn Debug {
    let return_value: dyn Debug = {
        let _buf1 = [0u8; 1 << 10];
        "hello world"
    };
    return_value
}
