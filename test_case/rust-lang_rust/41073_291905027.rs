rust
#![feature(untagged_unions)]
union U {
    x: (Box<u32>, Box<u32>),
    y: Vec<u32>,
    raw: [u64; 4]
}

fn main() {
    // init u.raw -> init (u.x, u.y) -> init u
    let mut u = U { raw: [0; 4] };
    unsafe {
        // uninit u.x.1 -> uninit u.x -> uninit (u.y, u.raw) -> uninit u
        // u.x.0 is still initialized
        drop(u.x.1);
        if true {
            // init u.y -> init (u.x, u.raw) -> init u
            // u.y was uninitialized, not dropped on assignment
            u.y = Vec::new(); // what does this drop?
        } else if true {
            // init u.x -> init (u.y, u.raw) -> init u
            // u.x was uninitialized, not dropped on assignment
            u.x = (Box::new(0), Box::new(1));
        } else {
            // assign u.x.0
            // u.x.0 was initialized, dropped on assignment
            u.x.0 = Box::new(0); // and this?
        }
    }
}
