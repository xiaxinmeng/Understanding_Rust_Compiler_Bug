 rust
error: missing type param `T` in the substitution of `T`
<anon>:1:42: 1:54 error: mismatched types: expected `A<T>` but found a ~-box pattern
<anon>:1 struct A<T> {a: T} fn f<T>(v: A<T>) {let ~A{a: ref a} = v;} fn main(){}
