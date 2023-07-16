
fn prefetch_data<T>(data: *const T, write: bool, locality: i32);
fn prefetch_instruction<T>(data: *const T, write: bool, locality: i32); // does T even make sense here?
