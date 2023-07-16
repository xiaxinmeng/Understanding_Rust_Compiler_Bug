
make[1]: Leaving directory '/home/jightuse/code/projects/rust/src/test/run-make/interdependent-c-libraries'

------ stderr ---------------------------------------------
bar.rs:13:1: 13:18 error: found staticlib `foo` instead of rlib `foo` , please compile using --crate-type rlib instead.
bar.rs:13 extern crate foo;
          ^~~~~~~~~~~~~~~~~
          error: aborting due to previous error
