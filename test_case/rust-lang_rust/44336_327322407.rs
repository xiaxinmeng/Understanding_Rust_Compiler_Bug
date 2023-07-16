rust
fn main() {
     let my_text = Some(String::from("my_text"));
     (move || {
         if let Some(s) = my_text {
              println!("Here it is: {}", s);
         }
     })();
}
