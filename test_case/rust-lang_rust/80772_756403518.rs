rust
trait SomeTrait {}

struct Exhibit {
    constant: bool,
    factory: fn(&usize) -> Box<dyn SomeTrait>,
}

const A_CONSTANT: &[Exhibit] = &[
    Exhibit {
        constant: "".is_empty(),
        factory: |_| unimplemented!(),
    },
    Exhibit {
        constant: "".is_empty(),
        factory: |_| unimplemented!(),
    },
];
