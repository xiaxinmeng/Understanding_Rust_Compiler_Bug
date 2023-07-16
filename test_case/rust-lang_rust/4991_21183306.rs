 rust
enum SmallVector<T> {
   Small(uint, [T, .. 8]), // size, data (the 8 shouldn't be hardcoded like this)
   Big(uint, uint, *T) // capacity, size, pointer
}
