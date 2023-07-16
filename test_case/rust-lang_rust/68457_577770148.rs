
[inline]  // error: expected item, found `[`
fn foo() {}

inline
fn foo() {}  // error: expected one of `!` or `::`, found keyword `fn`

@inline // error: expected item, found `@`
fn foo() {}

test! {  // error: cannot find macro `test` in this scope
    fn test_it_works() {
        assert_eq!(2 + 2, 4);
    }
}
