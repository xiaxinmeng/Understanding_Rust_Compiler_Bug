 rust
#![feature(slice_patterns)]

fn check(list: &[u8]) {
    match list {
        &[] => {},
        &[u1, u2, ref next..] => {},
        &[u1] => {},
    }
}
