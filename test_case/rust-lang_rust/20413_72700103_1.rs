 rust
/// Trait of transformation having a rotation extractable as a rotation matrix. This can typically
/// be implemented by quaternions to convert them to a rotation matrix.
pub trait RotationMatrix<N, LV, AV> : Rotation<AV> {
    type Output: Mat<N, LV, LV> + Rotation<AV>;

    /// Gets the rotation matrix represented by `self`.
    fn to_rot_mat(&self) -> Self::Output;
}
