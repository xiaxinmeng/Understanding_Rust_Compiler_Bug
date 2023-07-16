text
error[E0277]: the trait bound `nalgebra::base::matrix::Matrix<f32, nalgebra::base::dimension::U3, nalgebra::base::dimension::U1, nalgebra::base::array_storage::ArrayStorage<f32, nalgebra::base::dimension::U3, nalgebra::base::dimension::U1>>: std::convert::From<{integer}>` is not satisfied
 --> src/main.rs:4:5
  |
4 |     Into::<Vector3<f32>>::into(0);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<{integer}>` is not implemented for `nalgebra::base::matrix::Matrix<f32, nalgebra::base::dimension::U3, nalgebra::base::dimension::U1, nalgebra::base::array_storage::ArrayStorage<f32, nalgebra::base::dimension::U3, nalgebra::base::dimension::U1>>`
