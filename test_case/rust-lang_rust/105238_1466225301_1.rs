rust
#![feature(generic_const_exprs)]

struct Cond<const PRED: bool, U, V>(stdmarkerPhantomData<U>);

struct RobinHashTable<const MAX_LENGTH: usize, CellIdx = Cond<{ MAX_LENGTH < 65535 }, u16, u32>> {
    _idx: CellIdxR,
}

fn main() {
    println!("{}", std::mem::size_of::<RobinHashTable>);
}
