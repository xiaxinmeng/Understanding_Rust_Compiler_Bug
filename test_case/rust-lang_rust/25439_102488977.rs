 rust
struct Helper<'a>(&'a Fn(Helper, i32) -> i32);

fn fix<F>(f: F) -> i32
  where F: Fn(Helper, i32) -> i32      
{
  f(Helper(&f), 8) 
}

fn main() {  
  fix(|_,x| x);  
}
