rust
union A { data: [f32; 4], vec: f32x4, }
let x: [f32; 4] = unsafe { A { vec: f32x4::new(1., 2., 3., 4.) }.data };
assert_eq!(x[2], 3.);
