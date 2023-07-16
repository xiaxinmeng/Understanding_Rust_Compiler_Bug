rust
fn main() {
    // using println instead of assert_eq! to make sure it isn't just optimized
    // away as undefined behavior
    println!("{:?}", <i8>::SIZE); // prints 1
    println!("{:?}", <i16>::SIZE); // prints 2
    println!("{:?}", <i32>::SIZE); // prints 4
    println!("{:?}", <i64>::SIZE); // prints 8
    println!("{:?}", <Vec<()>>::SIZE); // prints 24
    println!("{:?}", <Vec<i8>>::SIZE); // prints 24

    fn test_vec<T>() {
        println!("{:?}", Vec::<T>::SIZE);
    }
    fn test_tuple<T>() {
        println!("{:?}", <(T, T)>::SIZE);
    }
    test_vec::<()>(); // prints 24
    test_vec::<i8>(); // prints 24
    test_tuple::<()>(); // prints 0
    test_tuple::<i8>(); // prints 2
}
