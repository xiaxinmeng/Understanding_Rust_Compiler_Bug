
error[E0759]: `animation_query` has an anonymous lifetime `'_` but it needs to satisfy a `'static` lifetime requirement
   --> src\main.rs:8:46
    |
8   | pub fn animation_system(mut animation_query: Query<(&mut Animations, &mut Handle<Image>)>) {}
    |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |                                              |
    |                                              this data with an anonymous lifetime `'_`...
    |                                              ...is used and required to live as long as `'static` here
    |
note: `'static` lifetime requirement introduced by this bound

