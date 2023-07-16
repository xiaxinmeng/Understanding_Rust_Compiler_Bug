 rust
let mm: MemoryMap<MyStructs> = MemoryMap::with_capacity(20) // Create a file and allocate 20 MyStructs worth of space.
// Use mm...
{
   let file = File::from_raw_fd(mm)
   // write to the file
}
// Use mm...
// SEGFAULT because writing to mm is equivalent to transmuting `[u8]` to `MyStruct`
