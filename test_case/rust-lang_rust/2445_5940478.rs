
class c1<T: copy> {
    new() {}

    fn f1(x: int) {
    }
}

impl i1<T: copy> for c1<T> {
    fn f2(x: int) {
    }
}


fn main() {
    c1::<int>().f1(4);
    c1::<int>().f2(4); //comment out to prevent ICE
}
