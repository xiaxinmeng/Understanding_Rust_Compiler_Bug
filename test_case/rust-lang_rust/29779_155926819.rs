 rust
const A: u8 = 1<<2;
fn main(){
    match 4 {
        A => panic!("yay"),
        _ => panic!("nay"),
    }
}
