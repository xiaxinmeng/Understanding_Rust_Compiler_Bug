 rust
fn words<'a>() -> Vec<&'a str>
{
    vec!["these","are","my","words"]
}

fn main() {
    let my = "my".into_string();
    if words().contains(&my.as_slice()) {
        println!("Its in there!");
    }
    else {
        println!("Word not found");
    }
}
