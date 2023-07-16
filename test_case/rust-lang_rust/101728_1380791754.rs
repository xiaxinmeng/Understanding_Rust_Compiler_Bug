rust
pub fn type_ascribe<T>(value: T) -> T { value }

#[allow(unused_macros)]
macro_rules! type_ascribe {
    ($e:expr, $t:ty) => {type_ascribe::<$t>($e)};
}

pub fn test(x: usize) {
    println!("{}", type_ascribe!(String::from("hello world").split_once(' '), _).unwrap().0);
    let v = type_ascribe!([[0, 1, 2, 3], [4, 5, 6, 7]][x], [u32; 4]);
}
