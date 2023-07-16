rust
match opt { // covered from branch out of `match`
    Some(1) => // `Some(1)` is covered by the `discriminant(_)` rvalue test
               // `1` is covered by the test for `1` or `2` in the pattern match
        1, // covered because of the `_n = const 1` assignment
    Some(2) => // NOT COVERED
        2, // covered because of the `_n = const 2` assignment
    None => // NOT COVERED
        0, // covered because of the `_0 = const 0` assignment
}
