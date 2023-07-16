
struct S;

mod m {
    use super::{self as root};
                ^^^^^^^^^^^^ no `super` in the root

    type A = root::S;
}
