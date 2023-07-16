 rust
macro_rules! m {
    ($i:ident) => {
        let mut $i = 2;
        $i = $i + 1;
    }
}

fn no_macro() {
    let mut a = 1;
    let mut a = 2;
    a = a + 1;

    println!("{}", a);
}

fn yes_macro() {
    let mut a = 1;
    m!(a);

    println!("{}", a);
}
