
fn main() {
    let my_text = String::from("My text");
    let runner = move || {
        println!("{}", my_text);
    };
    runner();
    runner();
}
