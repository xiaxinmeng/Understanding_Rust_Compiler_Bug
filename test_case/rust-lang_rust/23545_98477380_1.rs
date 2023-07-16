 rust
[1., 2., 3.].iter().fold(1./0., |acc, &x| f32::min(acc, x)); // OK
[1., 2., 3.].iter().fold(1./0., |acc, &x| acc.min(x)); // error: type `_` does not implement any method in scope named `min`
[1.0_f32, 2., 3.].iter().fold(1./0., |acc, &x| acc.min(x)); // error: type `_` does not implement any method in scope named `min`
