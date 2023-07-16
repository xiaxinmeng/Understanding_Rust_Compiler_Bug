text
error[E0277]: the trait bound `PathBuf: From<Cow<'_, str>>` is not satisfied
 --> src/lib.rs:6:5
  |
6 |     func(Path::new("hello").to_path_buf().to_string_lossy(), "world")
  |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `From<Cow<'_, str>>` is not implemented for `PathBuf`
  |
  = help: the following implementations were found:
            <PathBuf as From<&T>>
            <PathBuf as From<Box<Path>>>
            <PathBuf as From<Cow<'a, Path>>>
            <PathBuf as From<OsString>>
            <PathBuf as From<String>>
  = note: required because of the requirements on the impl of `Into<PathBuf>` for `Cow<'_, str>`
note: required by a bound in `func`
 --> src/lib.rs:3:20
  |
3 | fn func(path: impl Into<PathBuf>, code: impl Into<String>) {}
  |                    ^^^^^^^^^^^^^ required by this bound in `func`
