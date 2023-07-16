rust
// won't unwind
let count = (*other).len();
// could unwind, but nothing changes, data remains valid
self.reserve(count);
// won't unwind
let len = self.len();
// won't unwind
ptr::copy_nonoverlapping(other as *const T, self.as_mut_ptr().add(len), count); 
// could unwind (panic on overflow), but structures remain well formed, 
// as the data copied from the other vector is not considered "used"
self.len += count;
// won't unwind, though this could be changed to `other.len = 0` to make sure of it.
// Alternatively this could be swapped with the statement above for more confidence.
other.set_len(0);
