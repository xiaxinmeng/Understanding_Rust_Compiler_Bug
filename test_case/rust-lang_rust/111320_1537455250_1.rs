rust
// crate b
use a::{Wrapper, ImplPrivTrait, PubTrait};

pub fn foo(x: Wrapper<ImplPrivTrait>) {
    x.pub_fn();
}
