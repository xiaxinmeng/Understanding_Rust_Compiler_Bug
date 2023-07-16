
error[E0428]: the name `ItemKind` is defined multiple times
   --> src/librustdoc/json/types.rs:188:1
    |
158 | pub enum ItemKind {
    | ----------------- previous definition of the type `ItemKind` here
...
188 | pub enum ItemKind {
    | ^^^^^^^^^^^^^^^^^ `ItemKind` redefined here
    |
    = note: `ItemKind` must be defined only once in the type namespace of this module
