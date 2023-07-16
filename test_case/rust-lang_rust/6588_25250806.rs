 rust
struct A<T> { contents: ~[~[T]] }

impl<T: Clone> Index<(uint, uint),T> for A<T> {
    fn index(&self, ij: &(uint, uint)) -> T {
        match ij {
            &(i,j) => self.contents[i][j].clone()
        }
    }
}

fn main( ){
    let a = A{ contents: ~[~[1,2], ~[3,4]] };
    println(fmt!("%?", a[(1u,1u)]))
}
