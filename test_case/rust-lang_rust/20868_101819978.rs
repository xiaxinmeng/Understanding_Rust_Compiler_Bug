
iss_20868.rs:3:24: 5:6 error: closure may outlive the current function, but it borrows `x`, which is owned by the current function [E0373]
iss_20868.rs:3     std::thread::spawn(|| {
iss_20868.rs:4         let _ = x;
iss_20868.rs:5     });
iss_20868.rs:4:17: 4:18 note: `x` is borrowed here
iss_20868.rs:4         let _ = x;
                               ^
iss_20868.rs:3:24: 5:6 help: to force the closure to take ownership of `x` (and any other referenced variables), use the `move` keyword, as shown:
iss_20868.rs:      std::thread::spawn(move || {
iss_20868.rs:          let _ = x;
iss_20868.rs:      });
error: aborting due to previous error
