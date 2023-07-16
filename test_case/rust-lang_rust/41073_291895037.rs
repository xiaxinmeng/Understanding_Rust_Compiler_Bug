Rust
#![feature(untagged_unions)]
union U {
    x: (Box<u32>, Box<u32>),
    y: Vec<u32>,
    raw: [u64; 4]
}

fn main() {
    let mut u = U { raw: [0; 4] };
    unsafe {
        drop(u.x.1);
        if true {
            u.y = Vec::new(); // what does this drop?
        } else if true {
            u.x = (Box::new(0), Box::new(1));
        } else {
            u.x.0 = Box::new(0); // and this?
        }
    }
}
