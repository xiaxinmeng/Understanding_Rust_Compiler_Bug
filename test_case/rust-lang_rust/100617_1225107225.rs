
5 |     takes_option(*none);    // Suggests `*none.as_ref()`
  |     ------------ ^^^^^^^^^^^^^^
  |     |            |
  |     |            expected enum `Option`, found `&Option<String>`
  |     |            help: you can convert from `&Option<T>` to `Option<&T>` using `.as_ref()`: `*none.as_ref()`
  |     arguments to this function are incorrect
  |
  = note:   expected enum `Option<&String>`
          found reference `&Option<String>`
