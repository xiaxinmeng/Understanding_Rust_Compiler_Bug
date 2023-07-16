rust
#[repr(C)]
#[derive(Copy, Clone)]
struct Big { data: [i32; 5] }

#[repr(C)]
struct Small { data: i32 }

extern "C" {
    fn test(a: Big, b: Big, c: Big, d: Big, e: Big, f: Big, g: Small);
}

fn main() {
    let big = Big { data: [0; 5] };
    unsafe {
        test(big, big, big, big, big, big, Small { data: 42 });
    }
}
