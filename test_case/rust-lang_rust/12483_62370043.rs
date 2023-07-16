
match 1i {
    n if n > 0  => "greater",
    n if n < 0  => "less",
    _ => "equal",
}
