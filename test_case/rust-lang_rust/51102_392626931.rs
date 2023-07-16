rust
enum SimpleEnum {
    NoState,
}

fn main() {
    let _ = |simple| {
        match simple {
            SimpleEnum::NoState {
                state: 0
            } => (),
        }
    };
}
