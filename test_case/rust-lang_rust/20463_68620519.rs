 rust
let data = [0.0f32, 0.0];
black_box(&mut data); // doesn't mutate, but LLVM doesn't know
perlin2(&seed, &data);
