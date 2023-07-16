rust
static SLOW_RELOCATIONS: [&str; 50000] = ["hello", "hello", .... ];
static SLOW_FIELDS: [&i32; 50000] = [&("bar", 42, "foo", 3.14).1, .... ];
static SLOW_CHECKED_INDEX: [u8; 50000] = [b"foomp"[3], ... ];
static UNSIZING: [&[u8]; 50000] = [b"foo", ...];
trait Trait{}
impl Trait for u32 {}
static UNSIZE_TRAIT: [&Trait; 50000] = [&42u32, ...];
static CHAIN: [usize; 50000] = [42i32 as u8 as u64 as i8 as isize as usize, ...];
static OPS: [i32; 50000] = [((((10 >> 1) + 3) * 7) / 2 - 12) << 4, ...];
static FORCE_ALLOC: [i32; 50000] = [*****(&&&&&5), ...];
const fn nop<T>(t: T) -> T { t }
static CONST_FN_BASELINE: [i32; 50000] = [nop(42), nop(43), ...];
const fn inc(i: i32) -> i32 { i + 1 }
static CONST_FN_SIMPLE: [i32; 50000] = [inc(42), inc(43), ...];
