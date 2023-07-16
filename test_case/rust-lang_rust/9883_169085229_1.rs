 rust
fn deref_borrowed_box<T>(b: &Box<T>) -> &T { *b }
fn deref_borrowed_borrow<T, 'a, 'b>(b: &'a &'b mut T) -> &'a T { *b }
