
foldr.rs:11:11: 11:53 error: mismatched types: expected `std::iterator::UnfoldrIterator<,uint,(uint,int)>` but found `std::iterator::UnfoldrIterator<,(uint,int),(uint,int)>` (expected uint but found tuple)
foldr.rs:11     return UnfoldrIterator::new(next, (start, step));
                      ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to previous error
