
#![allow(dead_code)]
#![allow(unused_variables)]
enum Enum<F> {
    Tuple(F),
    Struct { f: F },
}
fn main(){
    let v = 42u32;
    let t = Enum::<u32>::Struct { f: v };
}
