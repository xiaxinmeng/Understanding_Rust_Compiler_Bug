rust
error[E0271]: type mismatch resolving `<Cell<T> as Pointee>::Metadata == ()`
   --> library/core/src/cell.rs:260:18
    |
260 | impl<T: Default> Default for Cell<T> {
    |                  ^^^^^^^ expected associated type, found `()`
    |
    = note: expected associated type `<Cell<T> as Pointee>::Metadata`
                     found unit type `()`
    = help: consider constraining the associated type `<Cell<T> as Pointee>::Metadata` to `()`
    = note: for more information, visit https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
