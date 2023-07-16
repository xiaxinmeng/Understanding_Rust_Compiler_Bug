rust
const fn less_than<const A: usize, const B: usize>() -> bool { A < B }

fn foo<const N: usize>()
    where ???
{
    let a: [u8; {2*N}] = ..;
    let b: [u8; {4*N}] = ..;
}
