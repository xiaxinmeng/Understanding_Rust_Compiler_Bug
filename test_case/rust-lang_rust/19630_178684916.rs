
macro_rules! nth(
        ($n:expr,$x:ident) => ($x.$n);
        );

fn main(){
        let foo = (2, 3, 4);
            let bar = nth!(2, foo);
}
