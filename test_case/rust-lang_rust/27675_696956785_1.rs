rust
fn main() {
    let static_word = {
        let st = String::from("Hello!");
        safe_transmute::<_, &'static mut str>(&*st)
    };
    
    println!("{:?}", static_word);
}
