rust
match opt { // covered from branch out of `match`
    Some(_) => // covered because of the `discriminant(_)` rvalue
        1, // covered because of the `_n = const 1` assignment
    None => // NOT COVERED
        0, // covered because of the `_n = const 0` assignment
}
