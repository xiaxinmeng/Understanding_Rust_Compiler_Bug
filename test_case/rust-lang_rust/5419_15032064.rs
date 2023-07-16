
foo.rs:12:3: 12:7 error: illegal by-move capture of captured outer immutable variable in a stack closure
foo.rs:12 do marc.access |v| { *v += 1; }
