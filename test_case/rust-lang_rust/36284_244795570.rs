
vec.shrink_to_fit();
let ptr = vec.as_mut_ptr();
let len = vec.len();
::std::mem::forget(vec);
// <snip>
drop(Vec::from_raw_parts(ptr, len, len));
