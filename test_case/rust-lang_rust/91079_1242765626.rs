rust
#[inline(never)]
fn hardware_max(a: f32, b: f32) -> f32 {
    a.max(b)
}

#[inline(always)]
fn llvm_evaluated_max(a: f32, b: f32) -> f32 {
    a.max(b)
}

fn main() {
    dbg!(hardware_max(0.0, 0.0).is_sign_positive());
    dbg!(hardware_max(0.0, -0.0).is_sign_positive());
    dbg!(hardware_max(-0.0, 0.0).is_sign_positive());
    dbg!(hardware_max(-0.0, -0.0).is_sign_positive());
    println!();
    dbg!(llvm_evaluated_max(0.0, 0.0).is_sign_positive());
    dbg!(llvm_evaluated_max(0.0, -0.0).is_sign_positive());
    dbg!(llvm_evaluated_max(-0.0, 0.0).is_sign_positive());
    dbg!(llvm_evaluated_max(-0.0, -0.0).is_sign_positive());
}
