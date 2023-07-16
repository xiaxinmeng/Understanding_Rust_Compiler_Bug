rust
 65     #[inline]
 66     fn lt(&self, __arg_0: &Foo<'a, T>) -> bool {
 67         match *__arg_0 {
 68             Foo(ref __self_1_0) =>
 69             match *self {
 70                 Foo(ref __self_0_0) =>
 71                 (*__self_0_0) < (*__self_1_0) ||
 72                     !((*__self_1_0) < (*__self_0_0)) && false,
 73             },
 74         }
 75     }
