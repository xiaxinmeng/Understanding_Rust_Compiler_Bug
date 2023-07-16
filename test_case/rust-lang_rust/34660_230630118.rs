 rust
macro_rules m! { () => {
    let x = 0 //< This PR will not affect trailing let statements, i.e. this will still be allowed.
}}
fn main() { m!(); }
