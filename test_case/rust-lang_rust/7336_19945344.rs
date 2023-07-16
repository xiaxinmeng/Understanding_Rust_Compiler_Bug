 rust
fn main() {
    let mut state = 54321;
    let a = || {
        let statep = &mut state;
        let f = || { *statep += 1 };
        f
    };

    a()();
}
