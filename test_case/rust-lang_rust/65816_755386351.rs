rust
size_of::<T>() * old_len == size_of::<U>() * new_len && 
size_of::<T>() * old_capacity == size_of::<U>() * new_capacity &&
align_of::<T>() == align_of::<U>()