

error: a trailing `|` is not allowed in an or-pattern
  --> $DIR/remove-leading-vert.rs:48:11
   |
LL |     let a | = 0;
   |         - ^ help: remove the `|`
   |         |
   |         while parsing this or-pattern starting here

error: top-level or-patterns are not allowed in `let` bindings
  --> $DIR/remove-leading-vert.rs:48:9
   |
LL |     let a | = 0;
   |         ^^^
