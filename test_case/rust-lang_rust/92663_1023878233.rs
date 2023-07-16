diff
- impl               Write for Cursor<&mut Vec<u8>>
+ impl<A: Allocator> Write for Cursor<&mut Vec<u8, A>>

- impl               Write for Cursor<Vec<u8>>
+ impl<A: Allocator> Write for Cursor<Vec<u8, A>>

- impl               Write for Cursor<Box<[u8]>>
+ impl<A: Allocator> Write for Cursor<Box<[u8], A>>
