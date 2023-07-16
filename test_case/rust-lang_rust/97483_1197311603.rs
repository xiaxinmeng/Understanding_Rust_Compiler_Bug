rust
fn main() {
    let mut s: Vec<u8> = Vec::with_capacity(10);
    s.extend(b"hello");
    let ptr = s.as_ptr(); // This pointer stays valid...
    let other = s.as_mut_ptr(); // Across as as_mut_ptr()
    let v = s; // Across a move of the original container
    unsafe {
        *other = b'b'; // And across a write to an aliasing pointer
        dbg!(*ptr);
    }

    let mut s = String::with_capacity(10);
    s.push_str("hello");
    let ptr = s.as_ptr(); // This pointer...
    let other = s.as_mut_ptr(); // Is invalidated here (though this case might be fixed, UCG 113)
    let v = s; // But wouldn't be invalidated here (this invalidates other with field retagging)
    unsafe {
        *other = b'b'; // We can't even get a pointer to do this without invalidating `ptr`, and we
                       // can't do this write by casting `ptr` because it doesn't have write
                       // permission.
        dbg!(*ptr);
    }
}
