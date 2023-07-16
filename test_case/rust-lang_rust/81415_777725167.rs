rust
matches!(foo, Bar | Baz);

matches!(
    some_really_long_expression,
    | Bar
    | Baz
)

match foo {
    Bar | Baz => {},
    _ => {}
}

match foo {
    | Bar
    | Baz => {},
    _ => {}
}
