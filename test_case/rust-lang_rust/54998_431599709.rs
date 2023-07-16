
trait Blah {
    fn test(&self) where Self: Sized {
        let a : &Blah = self  as &Blah;
    }
}
