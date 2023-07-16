rust
struct BytesMut;

trait Buf {}

impl Buf for BytesMut {}
impl<'a, T: Buf> Buf for &'a mut T {}

fn do_more<B>(mut buf: B)
where
    B: Buf,
{
    do_more(&mut buf);
}

fn main() {
    do_more(&mut BytesMut)
}
