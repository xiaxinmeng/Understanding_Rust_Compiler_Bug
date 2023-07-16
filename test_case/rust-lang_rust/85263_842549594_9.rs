rust
match (Foo { bar: 42 }) {
    Foo { twoval: TwoVal::One | TwoVal::Two } => {
        // always run
    },
}
