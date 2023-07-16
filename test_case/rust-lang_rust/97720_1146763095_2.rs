rust
fn foo(t: impl Foo<Assoc<'_> = Bar<'_>>) {}
                         ^^        ^^
                         ERROR     OK
