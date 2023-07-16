rust
// () does not implement Foo

type AlwaysInvalid = <() as Foo>::Bar; // no error...

struct Baz {
    baz: AlwaysInvalid // ...until it is used: only now will the compiler complain about (): Foo not being satisfied
}
