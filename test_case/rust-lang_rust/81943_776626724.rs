rust
fn main() {
    let sum: i32 = [1, 2, 3]
        .iter()
        .inspect(|n| match n {
            tmp => {
                ... macro-generated stuff here ...
                tmp
            }
        })
        .sum();
}
