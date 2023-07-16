
iss_19950_2.rs:23:5: 23:18 error: the trait `core::marker::Send` is not implemented for the type `*const ()` [E0277]
iss_19950_2.rs:23     thread::spawn(|| {
                      ^~~~~~~~~~~~~
iss_19950_2.rs:23:5: 23:18 note: `*const ()` cannot be sent between threads safely
iss_19950_2.rs:23     thread::spawn(|| {
                      ^~~~~~~~~~~~~
error: aborting due to previous error
