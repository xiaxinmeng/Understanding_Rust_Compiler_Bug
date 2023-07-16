rust
pub existential type Foo: Debug;

pub fn foo() -> Foo {
    5
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_foo() {
        assert_send(foo());
    }
}
