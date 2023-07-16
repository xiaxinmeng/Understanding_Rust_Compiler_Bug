 rust
trait Resources {
    type Buffer: Copy;
}

struct BufferHandle<R: Resources> {
    raw: <R as Resources>::Buffer,
}
impl<R: Resources> Copy for BufferHandle<R> {}

#[derive(Copy)]
enum Res {}
impl Resources for Res {
    type Buffer = u32;
}

fn main() {
    let b: BufferHandle<Res> = BufferHandle { raw: 1 };
    let c = b;
    print!("{}{}", c.raw, b.raw);
}
