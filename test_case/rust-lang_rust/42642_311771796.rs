rust
fn main() {
    let ex = |x| { // Error is thrown for x and below error is missing.
        let_(add(x,x), |y| { //~ ERROR type annotations needed
            let_(add(x, x), |x|x)})};
}
