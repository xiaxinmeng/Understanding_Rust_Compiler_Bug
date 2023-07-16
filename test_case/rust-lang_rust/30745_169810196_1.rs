 rust
fn main(){
    let x = (vec![0],);
    (|| { 
        drop(x.0); // cannot move out of captured outer variable in an `Fn` closure [E0507]
    })()
}
