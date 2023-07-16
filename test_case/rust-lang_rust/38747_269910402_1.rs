rust
use std::iter::FromIterator;

struct MyVec<T>(Vec<T>);

impl<T> FromIterator<T> for MyVec<T> {
    fn from_iter<I>(iter: I) -> Self 
        where I: IntoIterator<Item=T>
    {
        println!("allocating space for 100 elements");
        let mut this = MyVec(Vec::with_capacity(100));
        for elem in iter {
            this.0.push(elem);
        }
        this
    }
}

fn main() {
    let _: MyVec<i32> = None.into_iter().collect();
}
