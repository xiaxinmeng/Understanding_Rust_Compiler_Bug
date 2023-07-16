rust
loop {
    std::mem::forget(({Print("a")}, Print("b"), Print("c"), break));
}
