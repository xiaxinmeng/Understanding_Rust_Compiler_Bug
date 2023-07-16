rust
struct T;
struct U(i32);
struct R(T);

#[macro_export]
macro_rules! test {
  // First arm
  ($array_type:ty => $($elem:expr),+ $(,)?) => {};    
  // Second arm. Should be tried if the first fails, right? Macros are a match-like thing after all
  ($($elem:expr),+) => {println!("Expression branch")};  
}

fn main() {
    // Doesn't get rejected. Rust tries the second arm after realizing the first arm failed
    test!(T); 
    // Gets rejected. Rust should try all the arms, but terminates after failing with the first arm
    test!(U(2));
    // Doesn't get rejected purely because R(T) parses as a type 
    test!(R(T));
}
