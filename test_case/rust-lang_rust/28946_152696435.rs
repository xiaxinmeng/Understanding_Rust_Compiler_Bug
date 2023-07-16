
src\vector.rs:91:15: 91:24 error: overflow evaluating the requirement `<&vector::Vector3 as Dot<_>>::Output` [E0275]
src\vector.rs:91         (self.dot(self) - 1.0).is_zero()
                               ^~~~~~~~~
src\vector.rs:91:15: 91:24 help: run `rustc --explain E0275` to see a detailed explanation
src\vector.rs:91:15: 91:24 note: consider adding a `#![recursion_limit="128"]` attribute to your crate
