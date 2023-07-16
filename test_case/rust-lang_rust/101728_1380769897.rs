rust
pub fn type_ascribe<T>(value: T) -> T { value }

#[allow(unused_macros)]
macro_rules! type_ascribe {
    ($e:expr, $t:ty) => {{
        let val: $t = $e;
        val
    }};
}

pub fn main() {
    // macro dosn't work because of temporaries
    // println!("{}", type_ascribe!(String::from("hello world").split_once(' '), _).unwrap().0)
    
    //  identity function with turbo fish works
    println!("{}", type_ascribe::<_>(String::from("hello world").split_once(' ')).unwrap().0)
}
