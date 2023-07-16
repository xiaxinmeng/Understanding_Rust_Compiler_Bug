rust
const fn read<T: Copy>(r: &T) -> T { *r }
static FOO: u32 = read(&BAR);
static BAR: u32: 10;
