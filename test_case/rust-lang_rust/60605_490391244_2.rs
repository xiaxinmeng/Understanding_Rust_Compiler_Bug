rust-lang
fn main() {
    let body = reqwest::get("https://www.rust-lang.org");
    println!("Response: {:?}", body);
}
