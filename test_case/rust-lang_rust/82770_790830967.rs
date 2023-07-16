rust
#![feature(assert_matches)]
fn main() {
    let pair = (42, Some("asdf".to_string()));

    assert_matches!(pair.1, Some(_));
    dbg!(&pair); // OK, `pair` is still complete.

    let ref_pair = &pair;
    assert_matches!(ref_pair.1, Some(_));
    // error[E0507]: cannot move out of `ref_pair.1` which is behind a shared reference
    //  --> src/main.rs:9:21
    //   |
    // 9 |     assert_matches!(ref_pair.1, Some(_));
    //   |     ----------------^^^^^^^^^^-----------
    //   |     |
    //   |     data moved here
    //   |     move occurs because `left_val` has type `Option<String>`, which does not implement the `Copy` trait
    //   |
}
