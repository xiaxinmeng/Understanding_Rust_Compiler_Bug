
#![feature(const_fn, const_let)]

const fn churn_alloc_id() -> usize {
    let mut x: &i32 = &5;
    loop {
        x = &5;
    }
    0
}

fn main() {
    let _ = [(); churn_alloc_id()];
}
