 rust
mod foo {
    extern mod extra;
    use self::extra::ringbuf;
}

fn main() {}
