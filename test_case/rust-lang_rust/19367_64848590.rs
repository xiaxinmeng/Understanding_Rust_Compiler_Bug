 rust
struct S { o: Option<String> }

fn main() {
    let mut s = S{ o: Some("right".into_string()) };  
    let b = match s.o {
        Some(v) => {
            s.o = Some("wrong".into_string());
            v
        }
        None => String::new(),
    }; 
    println!("{}", b);
}
