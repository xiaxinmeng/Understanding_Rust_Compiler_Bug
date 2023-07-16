rust
    enum Void {}
    let r: Result<u32, Void> = Ok(4);
    let Ok(i) = r;
