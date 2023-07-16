rust
trait SomeTrait<T> {
    fn do_stuff(&self, value: T);
}

struct Structure;

impl Structure {
    fn do_stuff(&self, _: B) {
    }
}

// some example types
struct A;
struct B;
struct C;

impl SomeTrait<A> for &Structure {
    fn do_stuff(&self, _: A) {
    }
}

impl SomeTrait<C> for &Structure {
    fn do_stuff(&self, _: C) {
    }
}

let structure = Structure;
let tmp = &structure; // tmp is &Structure
let tmp = &tmp; // tmp is &&Structure
tmp.do_stuff(A);
tmp.do_stuff(C);

tmp.do_stuff(B); // compiler error
