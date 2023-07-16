rust
extern "C" fn foo(x: __m256);

fn main() {
    unsafe { 
        union U { v: __m256, a: [u64; 4] }
        foo(U { a: [0; 4] }.v);
    }
}
