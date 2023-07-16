rust
#![feature(object_safe_for_dispatch)]

trait Bad {
    fn bad_stat() { println!("trait bad_stat") }
    fn bad_virt(&self) { println!("trait bad_virt") }
    fn bad_indirect(&self) { Self::bad_stat() }
}

trait Good {
    fn good_virt(&self) { panic!() }
    fn good_indirect(&self) { panic!() }
}

impl<'a> Bad for dyn Bad + 'a {
    fn bad_stat() { println!("obj bad_stat") }
    fn bad_virt(&self) { println!("obj bad_virt") }
}

struct Struct {}

impl Bad for Struct {}

impl Good for Struct {}

fn main() {

    let s = Struct {};

    // Manually call static
    Struct::bad_stat();
    <dyn Bad>::bad_stat();

    let good: &dyn Good = &s;

    let bad = unsafe {std::mem::transmute::<&dyn Good, &dyn Bad>(good)};

    // Call virtual
    s.bad_virt();
    bad.bad_virt();

    // Indirectly call static
    s.bad_indirect();
    bad.bad_indirect();
    
}
