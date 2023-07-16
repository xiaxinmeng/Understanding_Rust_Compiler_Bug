rust
#![allow(unused_variables, unused_mut)]

#[derive(Debug)]
struct D(&'static str);

impl Drop for D {
    fn drop(&mut self) {
        println!("dropping D(\"{}\")", self.0);
    }
}

fn main() {
    let mut foo: Vec<&D> = Vec::new();
    {
        let block_temp = D("a block temp");
        println!("start of enclosing block");
        for (j, i) in (&vec![D("1"), D("2"), D("3")]).into_iter().enumerate() {
            println!("iter: {}", j);
            foo.push(i);
        }
        println!("end of enclosing block");
    }
}
