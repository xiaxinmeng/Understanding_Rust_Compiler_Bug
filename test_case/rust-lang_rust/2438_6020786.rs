
fn main() {
    let mut bar = [mut 0];
    vec::grow(bar, 128u, 0);
    bar[128] = 1;
    log(error, bar[128i8]);  // prints 1
}
