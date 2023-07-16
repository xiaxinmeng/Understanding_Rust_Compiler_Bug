
// check-pass

#[warn(unused_variables)]
#[warn(non_snake_case)]

fn main() {
    let unused: usize = 123;
    let WEIRD: usize = 456;
}
