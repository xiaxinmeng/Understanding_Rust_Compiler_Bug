rust
fn clone_something<T>(x: &T) -> T {
     <T as std::clone::Clone>::clone(x)
}

{
    struct Unclonable;
    clone_something(&Unclonable); // Erroring at this line. If it was removed, there'd be no error
}
