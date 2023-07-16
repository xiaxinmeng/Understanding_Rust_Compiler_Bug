 rust
fn foo(a: &'b io::Writer) {      
}                                

fn bar(b: &'b io::BytesWriter) { 
  foo(b as &'b io::Writer);      
}                                

fn main() {                      
}                                
