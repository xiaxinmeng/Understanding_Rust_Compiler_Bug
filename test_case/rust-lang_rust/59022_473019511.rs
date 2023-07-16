rust
fn constrain_mut<F: FnMut()>(f: F) -> F { f }
fn constrain<F: Fn()>(f: F) -> F { f }

fn main() {
    // This is fine:
    constrain_mut(constrain(|| {}));
    
    // This complains that `Fn` is not implemented:
    constrain(constrain_mut(|| {}));
}
