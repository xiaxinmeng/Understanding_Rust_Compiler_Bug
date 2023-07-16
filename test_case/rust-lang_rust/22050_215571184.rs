 rust
//extern crate num;
extern crate nalgebra;

pub fn fps_look_at<T: nalgebra::BaseFloat>(eye: Vector3<T>, pitch: T, yaw: T) -> Matrix4<T> {
    let (sin_pitch, cos_pitch) = pitch.sin_cos();
}
