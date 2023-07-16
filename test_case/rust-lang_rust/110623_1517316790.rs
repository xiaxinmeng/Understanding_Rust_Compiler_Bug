
trait S {}

fn foo<'a>(x: impl S + 'a) -> impl S + 'a {
    foo(foo(x))
}
