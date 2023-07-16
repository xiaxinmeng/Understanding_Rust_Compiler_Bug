 rust
spawn(proc() {
    while !*done.lock()) {}
})
