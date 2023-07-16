 rust
fn main(){
    let x = (vec![0],);
    (|| { 
        drop(x); // takes `x` by-value, therefore closure captures `x` by-value as well.
    })()
}
