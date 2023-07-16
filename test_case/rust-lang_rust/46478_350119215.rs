rust
macro_rules! hygiene_outer {
    () => {
        macro_rules! hygiene {
            () => { x }
        }
    }
}

fn main() {
    let x = 0usize;
    hygiene_outer!();
    println!("{:?}", hygiene!());
}

