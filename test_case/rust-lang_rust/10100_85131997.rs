
foo.rs:4:13: 4:31 error: import `Speed` conflicts with existing submodule [E0256]
foo.rs:4     pub use self::Speed::Speed;
                     ^~~~~~~~~~~~~~~~~~
foo.rs:5:5: 10:6 note: note conflicting module here
foo.rs:5     pub mod Speed {
foo.rs:6         pub enum Speed {
foo.rs:7             Fast,
foo.rs:8             Slow
foo.rs:9         }
foo.rs:10     }
error: aborting due to previous error
