rust
fn main() {
    let a = 3;
    let mut c = move || {
        let a = a;
        yield;
        let b = a;
    };
    Pin::new(&mut c).resume(());
    _zzz();
}
