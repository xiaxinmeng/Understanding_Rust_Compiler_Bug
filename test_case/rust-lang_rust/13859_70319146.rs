
fn f(_a: &mut [int; 2]) {
}

fn g(a: &mut [int; 2]) {
    f(a);
    a[1] = 2;
}

fn main() {
    let mut a = [1, 2];
    g(&mut a);
}
