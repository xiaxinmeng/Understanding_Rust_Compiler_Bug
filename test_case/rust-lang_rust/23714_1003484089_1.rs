
error[E0277]: the trait bound `[Element]: ToOwned` is not satisfied in `Element`
 --> src/lib.rs:4:28
  |
4 |     children: Cow<'static, [Element]>,
  |                            ^^^^^^^^^ within `Element`, the trait `ToOwned` is not implemented for `[Element]`
  |
  = help: the following implementations were found:
            <[T] as ToOwned>
note: required because it appears within the type `Element`
 --> src/lib.rs:3:8
  |
3 | struct Element {
  |        ^^^^^^^
  = note: slice and array elements must have `Sized` type

For more information about this error, try `rustc --explain E0277`.
