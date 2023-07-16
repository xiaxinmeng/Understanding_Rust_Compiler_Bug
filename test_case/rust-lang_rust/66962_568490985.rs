rust
struct MyStruct<const SIZE: usize> {
    bitfield: [usize; SIZE / 64]
}
