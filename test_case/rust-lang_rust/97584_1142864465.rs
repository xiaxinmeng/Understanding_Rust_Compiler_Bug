rust
>mod foo {
>    pub struct Qux;
>}
>
>mod bar {
 >   pub struct Qux;
>}
>
>mod baz {
>    use foo::*;
>    use bar::*; // Ok, no name conflict.
>}
>