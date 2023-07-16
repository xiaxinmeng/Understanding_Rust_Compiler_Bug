rust
// force `g` to be `const` even though we don't need that
const fn exec_f(foo: const Foo);

// effect system
const<F> struct Foo {
    f: const<F> fn(),
    g: fn(), // if the library author forgot to add an effect to `g`
    // you can't write a const fn to call it even if you know about it
}
const<F> exec_f(foo: Foo<F>);

// where bounds which only matter at compile-time
const fn exec_f(foo: Foo) where foo.f: const fn();
