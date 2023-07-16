 rust
macro_rules! generate_tests {
    ($m:ident, $t:ty) => {
        #[cfg(test)]
        mod $m {
            #[test]
            fn test() { }
        }
    }
}

struct Foo;
generate_tests!(foo, Foo);
