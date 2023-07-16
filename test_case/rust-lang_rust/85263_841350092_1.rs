rust
match (Foo { bar: 42 }) {
    Foo { oneval: x @ OneVal::One } => { //~ ERROR access to union field is unsafe
        println!("reached!");
    },
}
