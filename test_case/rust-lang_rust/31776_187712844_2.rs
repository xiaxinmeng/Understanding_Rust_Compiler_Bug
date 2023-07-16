
trait Tr {
    type A;
    fn pull(&self) -> Self::A;
}
struct S;

mod m {
    fn f() {
        struct Z {
            field: u8
        }

        impl ::Tr for ::S {
            type A = Z;
            fn pull(&self) -> Self::A { Z{field: 10} }
        }
    }
}

type Tmp = <S as Tr>::A;

fn main() {
    let a = S.pull().field; // Privacy error unless `field: u8` is pub
}
