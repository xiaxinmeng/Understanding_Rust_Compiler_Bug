
error[E0446]: private type alias `Foo` in public interface
  --> src/lib.rs:8:1
   |
5  |   existential type Foo: std::fmt::Debug
   |   - `Foo` declared as private
...
8  | / pub fn foo() -> Foo {
9  | |     Bar(5)
10 | | }
   | |_^ can't leak private type alias
