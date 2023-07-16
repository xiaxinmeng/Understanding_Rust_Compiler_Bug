rust
#[target_feature(enable = "avx")]
extern "C" fn foo(x: __m256) -> __m256;

fn main() {
    unsafe { 
        union U { v: __m256, a: [u64; 4] }
        if is_x86_feature_detected!("avx") {
            foo(U { a: [0; 4] }.v);
        }
    }
}
