rust
#[target_feature(enable = "avx")]
fn add_op(value: f32) -> Box<dyn Fn(&mut [f32])> {
    use std::arch::x86_64::*;
    Box::new(move |x: &mut [f32]| {
        for s in x.chunks_mut(8) {
            unsafe {
                _mm256_storeu_ps(
                    s.as_mut_ptr(),
                    _mm256_add_ps(_mm256_loadu_ps(s.as_ptr()), _mm256_set1_ps(value)),
                );
            }
        }
    })
}
