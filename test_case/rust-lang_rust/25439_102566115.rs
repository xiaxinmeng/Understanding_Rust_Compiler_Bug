 rust
fn fix<F>(f: F) -> i32
  where F: Fn(&F, i32) -> i32      
{
  f(&f, 8) 
}

fn main() {  
  fix(|_,x| x);  
}
