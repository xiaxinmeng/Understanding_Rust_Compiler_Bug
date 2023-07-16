
trait A<X:Eq> { 
    fn foo(&self, _: X) -> bool; 
}

struct E<T> { f: T } 
impl<T:Eq> A<T> for E<T> { 
    fn foo(&self, r: T) -> bool { self.f == r } 
}

struct B<T>;
impl<T:Eq> B<T> {
    fn check(&self, _ : Box<A<T>>) {}
    fn bar(&self,r:T) {
        let matcher = box E { f: r };
        self.check(matcher);
    }   
}

fn main(){}
