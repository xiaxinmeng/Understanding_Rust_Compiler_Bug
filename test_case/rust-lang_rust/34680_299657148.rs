rust
const fn read<T: Copy>(r: &T) -> T { *r }

static FOO: i32 = 0;

static BAR: i32 = read(&FOO);
