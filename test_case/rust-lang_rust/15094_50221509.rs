
scratch$ rustc -v
rustc 0.12.0-pre-nightly (a4553453a 2014-07-25 00:36:11 +0000)

scratch$ rustc r15094.rs 
r15094.rs:33:5: 36:6 error: method `call` has an incompatible type for trait: expected "rust-call" fn but found "Rust" fn [E0053]
r15094.rs:33     fn call(&self, args: (A, )) -> R {
r15094.rs:34         let (y, ) = args;
r15094.rs:35         self.x + y
r15094.rs:36     }
error: aborting due to previous error

scratch$ rustc r15094.rs 
r15094.rs:10:5: 12:6 error: method `call` has an incompatible type for trait: expected "rust-call" fn but found "Rust" fn [E0053]
r15094.rs:10     fn call(&self, _args: ()) {
r15094.rs:11         println!("{}", self.x);
r15094.rs:12     }
error: aborting due to previous error
