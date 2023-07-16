rust
struct X;
struct Y;
struct Z;

pub fn join<A, B, RA, RB>(_oper_a: A, _oper_b: B) -> (RA, RB)
    where A: FnOnce() -> RA + Send,
          B: FnOnce() -> RB + Send,
          RA: Send,
          RB: Send
{
    loop { }
}


fn main() {
    let ((_x, _y), _z): (_, Z) = join(|| (X, Y),
                                      || Z);
}
