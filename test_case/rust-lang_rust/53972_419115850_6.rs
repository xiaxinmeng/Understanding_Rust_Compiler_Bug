rust
const fn exec_f(foo: Foo) {
    (foo.f)() // ERROR, do not know whether `foo.f` is a const fn
}
