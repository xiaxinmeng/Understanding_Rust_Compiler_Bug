
Instead, I have to explicitly dereference and borrow the closure in the loop, with (&mut *closure)(val).
