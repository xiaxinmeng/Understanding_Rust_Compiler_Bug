
match box Foo { x: String::new("hi") } {
    box Foo { x } => ... use x ...
}
