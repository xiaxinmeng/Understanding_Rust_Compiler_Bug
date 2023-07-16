rust
#[repr(align(1))] struct Align1;
#[repr(align(2))] struct Align2;
#[repr(align(4))] struct Align4;
#[repr(align(8))] struct Align8;

fn main() {
    let a1 = Box::new(Align1);
    let a2 = Box::new(Align2);
    let a4 = Box::new(Align4);
    let a8 = Box::new(Align8);
    
    let p1 = &*a1 as *const _ as usize;
    let p2 = &*a2 as *const _ as usize;
    let p4 = &*a4 as *const _ as usize;
    let p8 = &*a8 as *const _ as usize;
    
    assert_eq!((p1, p2, p4, p8), (1, 2, 4, 8));
}
