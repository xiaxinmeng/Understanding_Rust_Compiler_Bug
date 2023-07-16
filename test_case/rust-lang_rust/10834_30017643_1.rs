
struct Foo { v: Foo1, _nc: Noncopyable }
enum Foo1 { A(..), B(..) }
``

and then match on `x.v` instead of `x`.

I think it's not so common though to use enums as the "top-level" type
for unsafe things like `Rc` etc.
