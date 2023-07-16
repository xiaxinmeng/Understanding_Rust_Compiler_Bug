
error: call to unsafe function is unsafe and unsafe operations are not allowed in const fn
  --> foo/src/test.rs:18:5
   |
18 | /     transmute::<
19 | |         [u8; size_of::<BufferDescriptor>() * BUFFER_CT],
20 | |         [BufferDescriptor; BUFFER_CT]
21 | |     >([0u8; size_of::<BufferDescriptor>() * BUFFER_CT])
   | |____________________________________________________________^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
