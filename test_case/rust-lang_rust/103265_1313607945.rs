rust
trait Foo {
    ~const fn a() {} // const fn - must be const in a `const` implementation.
    fn b() {} // non const
}
