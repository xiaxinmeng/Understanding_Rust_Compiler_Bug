rust
pub fn bools(x: &Vec<bool>) {
    let binary = |i, a: &Vec<bool>| {
        a[i] && a[i+1] // ok
    };

    let unary = |i, a: &Vec<bool>| {
        !a[i] // cannot infer type
    };

    binary(0, x);
    unary(0, x);
}

pub fn ints(x: &Vec<isize>) {
    let binary = |i, a: &Vec<isize>| {
        a[i] + a[i+1] // ok
    };
    let unary = |i, a: &Vec<isize>| {
        -a[i] // cannot infer type
    };
    
    binary(0, x);
    unary(0, x);
}
