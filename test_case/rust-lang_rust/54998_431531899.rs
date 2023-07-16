
trait Blah {
    fn test(&self) {
        let a : &Blah = self  as &Blah;
                            ^ here can not generate vtable because we don't know Real type of Self
    }
    
    fn test2(& self) -> u8;
}
