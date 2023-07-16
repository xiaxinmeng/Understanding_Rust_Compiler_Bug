rust
struct Bar(i32);

fn main() {
   let f = Bar(3);
   
   // compiler error that could be clearer:
   if let Bar{} = f {
      println!("hello");
   }
}
