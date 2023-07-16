rust
#![feature(const_fn_trait_bound)]
#![feature(const_trait_impl)]

trait A: PartialEq {}

const fn equals_self<T: A + ~const PartialEq>(t: &T) -> bool {
    *t == *t
}
