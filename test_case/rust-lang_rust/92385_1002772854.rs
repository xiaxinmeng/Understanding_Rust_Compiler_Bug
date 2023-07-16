rust
#![feature(const_fn_trait_bound, const_precise_live_drops, const_trait_impl)]

pub const fn ok<T, E>(res: Result<T, E>) -> Option<T>
where
    E: ~const Drop,
{
    match res {
        Ok(x) => Some(x),
        // Actually works without the leading `_` also,
        // but then you get an "unused variable" lint.
        Err(_x) => None,
    }
}

pub const fn err<T, E>(res: Result<T, E>) -> Option<E>
where
    T: ~const Drop,
{
    match res {
        // Actually works without the leading `_` also,
        // but then you get an "unused variable" lint.
        Ok(_x) => None,
        Err(x) => Some(x),
    }
}

const A: Result<u32, &str> = Ok(2);
const B: Result<u32, &str> = Err("Nothing here");
const C: Option<u32> = ok(A);
const D: Option<u32> = ok(B);
const E: Option<&str> = err(A);
const F: Option<&str> = err(B);

fn main() {
    assert_eq!(C, Some(2));
    assert_eq!(D, None);
    assert_eq!(E, None);
    assert_eq!(F, Some("Nothing here"));
}
