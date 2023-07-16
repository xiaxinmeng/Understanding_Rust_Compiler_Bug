rust
pub fn print_const<const N: usize>() {
    println!("{}", N);
}

const N: usize = 5;

pub fn print_thing<T>() {
    print_const::<{N}>()
}
