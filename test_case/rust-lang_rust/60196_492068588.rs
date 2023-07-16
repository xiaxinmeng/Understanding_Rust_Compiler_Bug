
use std::borrow::Cow;

fn main() {
    let slice = [1, 2, 3];
    let cow = Cow::from(&slice[..]);
    let _owned = cow.to_owned();
}
