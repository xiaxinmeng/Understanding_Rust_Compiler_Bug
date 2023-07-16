
macro_rules! cell {
    ($value:expr) => { 0 };
    ($style:ident ?) => { 1 };
}

fn main() {
    println!("{}", cell!(t?));
}
