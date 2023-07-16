
error[E0107]: wrong number of type arguments: expected 0, found 1
 --> file2.rs:1:34
  |
1 | pub struct Foo { i: Box<Iterator<isize>> }
  |                                  ^^^^^ unexpected type argument

error[E0191]: the value of the associated type `Item` (from the trait `std::iter::Iterator`) must be specified
   --> file2.rs:1:25
    |
1   | pub struct Foo { i: Box<Iterator<isize>> }
    |                         ^^^^^^^^^^^^^^^ missing associated type `Item` value
    |
   ::: /Users/ekuber/personal/rust/src/libcore/iter/iterator.rs:104:5
    |
104 |     type Item;
    |     ---------- `Item` defined here
help: if you meant to assign the missing associated type, use the name
    |
1   | pub struct Foo { i: Box<Iterator<Item = isize>> }
    |                                  ^^^^^^^^^^^^

error: aborting due to 2 previous errors
