
macro_rules! add {
    ([$($function:tt)*] $value:tt) => {
        $($function)* { ($x:expr) => { $x + $value } }
    }
}

macro_rules! extract {
    (($($parameters:tt)*) => { $dollar:tt x + $value:tt }) => { $value }
}

fn main() {
    add!([macro_rules! add_one] 1);  // OK
    assert_eq!(add_one!(4), 5);
    println!("{}", add!([stringify!] 1));  // OK
    println!("{}", extract! { ($x:expr) => { $x + 1 } });  // OK
    println!("{}", add!([extract!] 1));  // not OK
}
