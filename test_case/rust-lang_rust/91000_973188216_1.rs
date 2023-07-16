rust
let mut ptr = start;
while ptr < end {
    unsafe { foreign(ptr) }
    ptr = unsafe { ptr.add(1) };
}
