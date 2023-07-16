rust
const foo: [extern fn(&mut u32); 1] = [
  |var: &mut u32| {},
];
