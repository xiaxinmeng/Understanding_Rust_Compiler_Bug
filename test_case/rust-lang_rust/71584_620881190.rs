
fn main() {
    if true {
        let f = |number: usize| number;
    } else {
        let confusion: usize = 0usize + 0usize.into();
    }
}

fn main2() {
    let f = |number: usize| number;
    let confusion: usize = 0usize + 0usize.into();
}

fn main3() {
    let x: usize = 0usize + 0usize.into();
}
