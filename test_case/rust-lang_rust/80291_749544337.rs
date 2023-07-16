Rust
trait ColumnOutput<'a, T> {
    type Output: Sized;
}

struct C;

impl<'a, T> ColumnOutput<'a, T> for C {
    type Output = T;
}

fn calc<'a, T1: Default, T2, O1, O2, F1>(f: F1)
where
    O1: ColumnOutput<'a, T1>,
    O2: ColumnOutput<'a, T2>,
    F1: Fn(&<O2 as ColumnOutput<'a, T2>>::Output) -> T1, //Lifetime added
{
    let c2_data: Vec<<O2 as ColumnOutput<T2>>::Output> = Vec::new();
    let mut dummy: Vec<T1> = Vec::new();
    dummy.extend(c2_data.iter().map(|c2_value| f(c2_value)));
}

pub fn run_calc() {
    calc::<u64, u64, C, C, _>(|c2_data| *c2_data)
}

fn main() {
    run_calc();
}
