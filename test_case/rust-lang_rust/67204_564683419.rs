rust
fn main() {}
fn bar() -> Result<(), ()> {
    panic!()
}

fn foo() -> Result<(), ()> {
   let x = async {
       bar()?;
   }; 
   Ok(())
}
