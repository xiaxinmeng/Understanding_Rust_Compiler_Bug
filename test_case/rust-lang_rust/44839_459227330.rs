rust
use std::arch::x86_64::*;

fn main() {
    if (is_x86_feature_detected!("sse4a")) {
        println!("{:?}", unsafe { _mm_extract_si64(_mm_setzero_si128(), _mm_setzero_si128()) });
    }
}
