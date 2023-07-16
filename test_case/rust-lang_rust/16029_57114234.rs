 rustc
extern crate rustc;

fn main() {
    let x = std::task::try(proc() {
        println!("hello!");
    });
    let y = match x {
        Ok(..) => "Ok!",
        Err(..) => "Err!",
    };
    println!("{}", y);
}
