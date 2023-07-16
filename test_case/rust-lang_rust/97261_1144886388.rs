
r = &mut nonnull.0;
(r as *mut usize).write(0); // possibly in another function
v = nonnull.0;
