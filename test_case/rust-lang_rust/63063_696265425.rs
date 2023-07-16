rust
type Callback = impl Fn();

fn callback() {
    return;
}

fn register(_cb: Callback) {
    return;
}

fn main() {
    let cb = || callback();
    register(cb);
}
