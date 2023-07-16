
macro_rules! m {
    ($arg1: ident, $arg2: ident) => {
        (move || {
            $arg1 += 1;
            println!("{:?}", $arg1);
            (move || {
                $arg2 += 1;
                println!("{:?}", $arg2);
            })();
        })();
    }
}


fn main() {
    let mut a = 0;
    let mut b = 0;
    println!("{:?}", a);
    m!(a, b);
}
