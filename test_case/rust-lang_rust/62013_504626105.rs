rust
pub struct Demo {
    foo: Option<Box<Demo>>
}

pub fn foo2_desugered<'a>(mut a: &'a mut Demo) {
    {
        let mut _t1 = &mut a.foo;
        
        // this will work
        a = _t1.as_mut().unwrap();
        
        // this will not
        // match _t1 {
        //     Some(_1) => a = _1,
        //     None => ()
        // }
    }
    a.foo = None;
}
