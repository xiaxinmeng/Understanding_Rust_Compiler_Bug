
18803.rs:33:29: 33:31 error: cannot borrow `gl` as mutable more than once at a time
18803.rs:33   let y = Shaders::new(&mut gl);
                                        ^~
18803.rs:32:29: 32:31 note: previous borrow of `gl` occurs here; the mutable borrow prevents subsequent moves, borrows, or modification of `gl` until the borrow ends
18803.rs:32   let x = Shaders::new(&mut gl);
                                        ^~
18803.rs:34:2: 34:2 note: previous borrow ends here
18803.rs:29 pub fn main() {
18803.rs:30   let mut gl = GLContext;
18803.rs:31 
18803.rs:32   let x = Shaders::new(&mut gl);
18803.rs:33   let y = Shaders::new(&mut gl);
18803.rs:34 }
            ^
error: aborting due to previous error
