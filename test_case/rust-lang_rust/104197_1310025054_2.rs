
error: lifetime may not live long enough
  --> src/main.rs:62:24
   |
61 | fn do_something<'a, P: View>(parent: &Hierarchical<P>) {
   |                              ------ has type `&Hierarchical<'1, P>`
62 |     let h: &'a dyn View = parent;
   |                           ^^^^^^ cast requires that `'1` must outlive `'a`
   = note: `dyn View` lifetime has been inferred as `dyn View + 'a`
