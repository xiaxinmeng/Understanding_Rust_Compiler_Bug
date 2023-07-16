
pub trait Float: Num {
    static pure fn NaN() -> Self;
    static pure fn infinity() -> Self;
    static pure fn neg_infinity() -> Self;

    pure fn is_infinite(&self) -> bool;
    pure fn is_finite(&self) -> bool;
    pure fn is_NaN(&self) -> bool;
    ...
}
