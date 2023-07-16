 rust
let trait_objects: [&Trait; 2] = [&A, &B];
assert_eq!(trait_objects[0].CONST, 'A');
assert_eq!(trait_objects[1].CONST, 'B');
