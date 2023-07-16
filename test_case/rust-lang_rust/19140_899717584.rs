rust
enum Mode {
    Open,
    Close
}

fn main() {
    let mode = Mode::Open;
    let num = match mode {
        Mode::Open => 1,
        Mode::Close => 2
    };
}
