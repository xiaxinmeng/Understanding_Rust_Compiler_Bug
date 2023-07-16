rust
use std::borrow::Cow;

struct Element {
    children: Cow<'static, [Element]>,
}
