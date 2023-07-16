rust
let mut data = [1, 2, 3];

let slice = &mut data[..];
let ptr1 = &mut slice[0]; // handing this out to the user

let slice = &mut data[..]; // Uh-oh! This slice *overlaps* with the reference the user got!
let ptr2 = &mut slice[1]; // handing this out to the user

// Now as a user, try using both ptr1 and ptr1...
mem::swap(ptr1, ptr2);
// The borrow checker says "no", because when we created the 2nd "slice", we invalidated "ptr1".
