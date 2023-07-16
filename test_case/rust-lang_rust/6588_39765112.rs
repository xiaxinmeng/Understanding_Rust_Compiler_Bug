 rust
struct A<T>(~[~[T]]);

impl<T: Copy> Index<(uint, uint),T> for A<T> {
    fn index(&self, ij: &(uint, uint)) -> T {
        let A(ref v) = *self;

        match ij {
            &(i,j) => (v)[i][j]
        }
    }
}

fn main( ){
    let a = A(~[~[1,2], ~[3,4]]);
    println!("{}", a[(1u,1u)])
}
