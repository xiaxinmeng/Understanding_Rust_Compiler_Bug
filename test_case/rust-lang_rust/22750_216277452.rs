

   Compiling test v0.1.0 (file:///test)
main.rs:17:37: 17:70 error: mismatched types:
 expected `&nalgebra::structs::vector::Vector3<f32>`,
    found `&nalgebra::structs::vector::Vector3<f32>`
(expected struct `nalgebra::structs::vector::Vector3`,
    found a different struct `nalgebra::structs::vector::Vector3`) [E0308]
main.rs:17         c.prepend_to_local_rotation(&Vector3::new(0.0f32, 0.014, 0.0));
                                               ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
main.rs:17:37: 17:70 help: run `rustc --explain E0308` to see a detailed explanation
main.rs:17:37: 17:70 note: Perhaps two different versions of crate `nalgebra` are being used?
main.rs:17         c.prepend_to_local_rotation(&Vector3::new(0.0f32, 0.014, 0.0));
                                               ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to previous error
Could not compile `test`.

