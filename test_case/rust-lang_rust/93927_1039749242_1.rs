rust
struct MyType<T>(T);

impl<T> PartialEq for MyType<T>
where
    T: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

fn cond<T: PartialEq>(val: MyType<T>) -> bool {
    val == val
}

fn main() {
    cond(MyType(0));
}
