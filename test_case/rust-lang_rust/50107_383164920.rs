rust
trait Trait {
    type A;
    type B;
}

enum Void {}
impl Trait for Void { type A = Void; type B = Void; }

enum Foo<A, B> { A(A), B(B) }
impl<A, B> Trait for Foo<A, B> { type A = A; type B = B; }

enum Many<A, B, T1 = Void, T2 = Void, T3 = Void>
where T1: Trait<A = A, B = B>,
      T2: Trait<A = A, B = B>,
      T3: Trait<A = A, B = B>,
{
    M1(T1), M2(T2), M3(T3)
}

fn main() {
    let x = match 3 {
        1 => Many::M1(Foo::A(3u8)),
        _ => Many::M2(Foo::B(3u16)),
    };
}
