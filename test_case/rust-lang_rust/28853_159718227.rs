 rust
use std::collections::HashMap;
pub fn named_lints<'a>(names: &[&str],
                       transforms: &'a HashMap<&'static str, u32>)
                       -> Option<&'a u32> {
    transforms.get(&names[0])
}
fn main(){}
