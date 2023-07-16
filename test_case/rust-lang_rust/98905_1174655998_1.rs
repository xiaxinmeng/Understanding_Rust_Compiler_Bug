text
error[E0277]: the trait bound `Cow<'_, CurDir>: AsRef<Path>` is not satisfied
  --> src/main.rs:23:10
   |
23 |     func(Cow::Borrowed(&CurDir));
   |     ---- ^^^^^^^^^^^^^^^^^^^^^^ the trait `AsRef<Path>` is not implemented for `Cow<'_, CurDir>`
   |     |
   |     required by a bound introduced by this call
   |
   = help: the following other types implement trait `AsRef<T>`:
             <Cow<'_, OsStr> as AsRef<Path>>
             <Cow<'_, T> as AsRef<T>>
note: required by a bound in `func`
  --> src/main.rs:13:17
   |
13 | fn func(_: impl AsRef<Path>) {}
   |                 ^^^^^^^^^^^ required by this bound in `func`

error[E0277]: the trait bound `Cow<'_, str>: AsRef<Path>` is not satisfied
  --> src/main.rs:26:10
   |
26 |     func(Cow::Borrowed(s));
   |     ---- ^^^^^^^^^^^^^^^^ the trait `AsRef<Path>` is not implemented for `Cow<'_, str>`
   |     |
   |     required by a bound introduced by this call
   |
   = help: the following other types implement trait `AsRef<T>`:
             <Cow<'_, OsStr> as AsRef<Path>>
             <Cow<'_, T> as AsRef<T>>
note: required by a bound in `func`
  --> src/main.rs:13:17
   |
13 | fn func(_: impl AsRef<Path>) {}
   |                 ^^^^^^^^^^^ required by this bound in `func`

For more information about this error, try `rustc --explain E0277`.
