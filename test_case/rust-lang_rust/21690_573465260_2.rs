rust
#[math(overflow="wrap", assumptions=("algebraic", "no-nan", "finite"))]
fn foo(b1: &[f32]) -> f32 {
    accurate::kahan_sum(b1)
}
