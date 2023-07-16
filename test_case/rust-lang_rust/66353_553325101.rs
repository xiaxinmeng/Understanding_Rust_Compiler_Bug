rust
use nalgebra::base::Vector3;

fn crash() {
    Into::<Vector3<f32>>::into(0);
}
