
pub trait Num: Ord, Eq, Add<self, self>, Sub<self, self>, Mul<self, self>, Div<self, self>, Modulo<self, self>, Neg<self> {
    pure fn to_int() -> int;
    static pure fn from_int(n: int) -> self;
}
