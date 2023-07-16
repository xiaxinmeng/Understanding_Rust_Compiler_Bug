rust
fn main(){
    const A: &str = "hello";
    const B: &str = " world";
    const C: &str = concat_strs!(A, B, B, B, B);
    dbg!(C);
}
