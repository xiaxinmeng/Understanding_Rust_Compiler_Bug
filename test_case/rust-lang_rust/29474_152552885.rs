 md
Note the documentation for the primitives [`str`](primitive.str.html)
and [`[T]`](primitive.slice.html) (also called 'slice'). Many method calls
on [`String`](string/struct.String.html) and [`Vec`](vec/struct.Vec.html)
are actually calls to methods on `str` and `[T]` respectively, via [deref
coercions](../book/deref-coercions.html).
