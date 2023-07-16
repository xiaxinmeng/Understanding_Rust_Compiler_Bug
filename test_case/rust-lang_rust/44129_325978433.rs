rust
fn main() {
    // This case illustrates one particular ordering of interest:
    D1(&S1("tmp"));
}

#[derive(Debug)]
struct S1(&'static str);

#[derive(Debug)]
struct D1<'a>(&'a S1);

impl<'a> Drop for D1<'a> {
    fn drop(&mut self) {
        println!("D1({:?})", self.0); 
    }
}
