rust
struct SimpleStruct {
    no_state_here: u64
}

fn main() {
    let _ = |simple| {
        match simple {
            SimpleStruct {
                state: 0
            } => (),
        }
    };
}
