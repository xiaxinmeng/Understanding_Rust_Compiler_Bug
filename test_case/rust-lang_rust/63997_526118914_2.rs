rust
struct Foo<T>(T);
trait Bar { const F: const fn(Self) -> Self; }

impl<T: Bar> Foo<T> {
    const fn new(x: T) -> Self { Foo(<T as Bar>::F(x)) }
}

const fn map_i32(x: i32) -> i32 { x * 2 }
impl Bar for i32 { const F: const fn(Self) -> Self = map_i32; } 
const fn map_u32(x: i32) -> i32 { x * 3 }
impl Bar for u32 { const F: const fn(Self) -> Self = map_u32; } 
