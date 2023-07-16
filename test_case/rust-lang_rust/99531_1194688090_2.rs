rust
for<'a> extern "C" fn(Foo<'a>) -> Bar<'a>
// desugars to:
FnPtr<
    for<'a> extern "C" k#fn_sig(Foo<'a>) -> Bar<'a>>,
>
