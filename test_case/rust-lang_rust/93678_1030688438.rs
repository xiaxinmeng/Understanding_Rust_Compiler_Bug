rs
fn f() {
    let x: [(); unsafe { size() }] = [];
}

const unsafe fn size() -> usize { 0 } 
