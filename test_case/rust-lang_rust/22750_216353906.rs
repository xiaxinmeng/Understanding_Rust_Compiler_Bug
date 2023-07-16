
   Compiling test v0.1.0 (file:///Users/alexhansen/Codin/rust/test)
main.rs:17:37: 17:70 error: mismatched types:
 expected `&nalgebra::structs::vector::Vector3<f32>`,
    found `&na::structs::vector::Vector3<f32>`
(expected struct `nalgebra::structs::vector::Vector3`,
    found struct `na::structs::vector::Vector3`) [E0308]
main.rs:17         c.prepend_to_local_rotation(&Vector3::new(0.0f32, 0.014, 0.0));
                                               ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

