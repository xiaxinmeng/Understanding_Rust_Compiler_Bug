
<anon>:7:6: 7:7 error: the type parameter `T` is not constrained by the impl trait, self type, or predicates [E0207]
<anon>:7 impl<T: Add<T>, V: Vector<T>> Add<V> for V {
              ^
