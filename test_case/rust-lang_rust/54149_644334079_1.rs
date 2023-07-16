rust
fn myfunc<I: Into<u32>, T: MyTrait2<X = I>>(v: &T) {
    println!("{}", Into::<u32>::into(v.get_val()));
}
