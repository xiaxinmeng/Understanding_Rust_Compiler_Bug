
% rustc +nightly -C no-prepopulate-passes "-Cpasses=  lint  "  ../../narrowed-48310.rs
% rustc +nightly -C no-prepopulate-passes "-Cpasses= early-cse lint  "  ../../narrowed-48310.rs
Undefined behavior: Call argument type mismatches callee parameter type
  invoke void bitcast (void (%Bar*)* @"_ZN59_$LT$narrowed_48310..Bar$u20$as$u20$narrowed_48310..Foo$GT$6borrow1\
7hdd495dfcc8d99ffcE" to void ({}*)*)({}* align 1 %14)
          to label %15 unwind label %18
