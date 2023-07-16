rust
let mut slice = std::slice::from_raw_parts_mut(...); // suddenly a pile of extremely subtle
                                                     // invariants in order for this to be allowed
for elem in slice {
    unsafe { cpp(elem as *mut T); } // probably UB
}