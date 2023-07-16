 rust
#[derive(Debug)]
enum E { C, A, B }
// force E to be zeroed when it goes out of scope
impl Drop for E { fn drop(&mut self) { } }

fn main() {
    let mut result = E::A;
    match result {
        mut x => {
            result = E::B;
            println!("{:p} {:p}", &mut x, &mut result);
        }
    }
    println!("result: {:?}", result);
}
