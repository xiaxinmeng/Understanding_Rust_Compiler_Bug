
fn main() {
    let a = 10;
    macro_rules! m {
        () => (
            let b = 11;
            macro_rules! n {
                () => {
                    (a, b)
                }
            }
        )
    }
    m!();
    let c = n!();
}
