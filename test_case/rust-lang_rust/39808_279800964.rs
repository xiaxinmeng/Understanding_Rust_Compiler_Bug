rust
use std::borrow::Cow;

fn main() {
    let _ = if false {
        Cow::Owned(format!("{:?}", panic!())) /* as Cow<str> */ // uncomment to fix
    } else {
        Cow::Borrowed("")
    };
}
