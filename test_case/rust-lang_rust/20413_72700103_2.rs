 rust
/// Builds a rotation matrix from `r`.
#[inline(always)]
pub fn to_rot_mat<N, LV, AV, R: RotationMatrix<N, LV, AV>>(r: &R) -> R::Output {
    r.to_rot_mat()
}
