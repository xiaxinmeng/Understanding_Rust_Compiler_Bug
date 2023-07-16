
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct TestStruct {
    u: u32,
}

fn main() {
    let mut val = TestStruct{ u: 1 };
    val.u += 2;
    println!("val {:?} ", val);
    //prints: val TestStruct { u: 3 }
    
    let mut val_rc = Rc::new(TestStruct{ u: 10 });    
    //above I mistakenly assumed "mut" would let me mutate the subcontents of the Rc.
    val_rc.u += 2;
    //above creates compile error: error[E0594]: cannot assign to data in a `&` reference
    println!("val_rc {:?} ", val_rc);    

    //The solution I used
    let val_rc_refcell = Rc::new(RefCell::new(TestStruct{ u: 20 }));
    (*val_rc_refcell).borrow_mut().u += 2;
    println!("val_rc_refcell {:?} ", val_rc_refcell);        
}

