 rust
fn divide_by_three(x: i32) -> i32 { // one of the poorest implementations of x/3
    for i in std::iter::count(0_i32, 1) {
        if i < 0 { panic!("i32 overflow"); }
        if x < 3*i { return i; }
    }
    unreachable!();
}

fn main() {
    println!("{}", divide_by_three(10));
}
