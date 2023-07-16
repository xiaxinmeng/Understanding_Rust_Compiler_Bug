rust
const fn bla<T>() -> usize { type_name::<T>().len() }

fn z(_: &'static [usize]) {}

fn f() {
    let x = read_integer_or_whatever();
    z(&[bla::<typeof(x)>()]);
}
