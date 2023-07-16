
2992.rs:2:5: 2:6 error: cannot capture variable of type `A`, which does not fulfill `'static`, in a bounded closure
2992.rs:2   || a.clone()
               ^
2992.rs:2:5: 2:6 note: this closure's environment must satisfy `'static`
2992.rs:2   || a.clone()
               ^
error: aborting due to previous error
