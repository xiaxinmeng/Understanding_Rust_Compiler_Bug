rust
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum BorrowckMode {
    Ast,
    Mir,
    Compare,
}
