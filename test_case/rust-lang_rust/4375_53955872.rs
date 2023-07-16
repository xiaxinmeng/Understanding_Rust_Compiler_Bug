 rust
macro_rules! do_bar(
    () => (
        let x = 1u;
        if x == 1 {return}
    )
)

pub fn main() {
    // error: macro expansion ignores token `if` and any following
    do_bar!();
} 
