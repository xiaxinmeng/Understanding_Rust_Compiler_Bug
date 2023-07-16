rust
const NUM_A: usize = 11;

#[derive(Clone, Copy)]
struct Foo {
    a: [&'static str; NUM_A],
}

const NUM_FOO: usize = 1500;

const FOOS: [Foo; NUM_FOO] = [Foo { a: [""; NUM_A] }; NUM_FOO];

#[cfg(test)]
mod tests {
    #[test]
    fn test_foo() {
        for f in super::FOOS {}
    }
}
