
pub trait Len {
    fn len(&self) -> usize;
}

fn collection_len<T:Len>(collection:&T) ->usize {
    collection.len()
}

impl<T, const c: usize> Len for [T; c] {
    fn len(&self) -> usize {
        c
    }
}

impl<T> Len for Vec<T> {
    fn len(&self) -> usize {
        self.len()
    }
}

fn main() {
    println!("{}",collection_len(&vec![1,2,3]));
    println!("{}",collection_len(&[1,2,3]));
}
