
error[E0119]: conflicting implementations of trait `HasFooIndirect` for type `WithFoo<_>`
   --> lib/shapely/impl/src/lib.rs:156:9
    |
153 | /         impl<T>
154 | |         [<Has $name Indirect>] for [<With $name>]<T> {}
    | |_______________________________________________________- first implementation here
