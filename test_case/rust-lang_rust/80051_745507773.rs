
trait ArrayLen { const LEN: usize; }
struct ArrayLenImpl<const N: usize>;
impl<const N: usize> ArrayLen for ArrayLenImpl<N> { const LEN: usize = N; }

type Magic = impl ArrayLen;
fn magic<T, const N: usize>(_: &[T; N]) -> Magic { ArrayLenImpl::<N> }

fn main() {
    let a = [0_u32; 10];
    magic(&a);
    let b = [0; Magic::LEN];
}
