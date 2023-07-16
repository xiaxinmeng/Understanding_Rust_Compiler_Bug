rust
fn call(_: String) {}

fn main() {
    if let Some(s) = Option::<String>::None {
        s == String::new();
    }
    let local_var: String;
    call(local_var);
}

