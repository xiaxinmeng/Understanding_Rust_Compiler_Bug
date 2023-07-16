 rust
fn deref_borrowed_box<T>(b: &mut Box<T>) -> &mut T { *b }
fn deref_borrowed_borrow<T, 'a, 'b>(b: &'a mut &'b mut T) -> &'a mut T { *b }
