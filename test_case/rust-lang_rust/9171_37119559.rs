 rustc
fn main() {
    macro_rules! breakme(
        ($value: expr) => {
            break 'foo;    // error: use of undeclared label `foo`
        }
    )
    'foo: loop { 
        breakme!("foo");
    }
}
