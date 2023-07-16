
error: reached the type-length limit while instantiating `alt::<&str, std::option::Option<...mnom/src/lib.rs:169:23: 169:40]>`
   --> omnom/src/lib.rs:122:1
    |
122 | / pub fn alt<I, T>(f: impl Omnom<I, T>, g: impl Omnom<I, T>) -> impl Omnom<I, T> {
123 | |     move |i| f(i).or_else(|(i, _)| g(i))
124 | | }
    | |_^
    |
    = note: consider adding a `#![type_length_limit="2683000"]` attribute to your crate
