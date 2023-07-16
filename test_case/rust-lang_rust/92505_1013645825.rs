rust
trait A<T> {
    type I;
    
     fn f() where Self::I: A<T> {}
}

impl<T> A<T> for () {
    type I = ();
    
     fn f() where Self::I: A<T> {
         <() as A<T>>::f();
     }
}
