
error: unresolved link to `Vec<Box<T>`
  --> $DIR/intra-link-malformed-generics.rs:5:6
   |
LL | //! [Vec<Box<T>]
   |      ^^^^^^^^^^ unbalanced angle brackets

warning: unclosed HTML tag `T`
  --> $DIR/intra-link-malformed-generics.rs:5:13
   |
LL | //! [Vec<Box<T>]
   |             ^^^
   |
   = note: `#[warn(invalid_html_tags)]` on by default
