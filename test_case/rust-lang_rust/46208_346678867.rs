
match self {
    &Animal::Cat(ref c) => f.write_str("c"),
    &Animal::Dog => f.write_str("d"),
}
