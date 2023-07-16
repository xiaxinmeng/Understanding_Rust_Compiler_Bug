rust
fn call<F: FnOnce()>(f: F) { f() }
fn main() {
    let my_text = Some(String::from("my_text"));
    call(move || {
        if let Some(s) = my_text {
            println!("Here it is: {}", s);
        }
    });
}
