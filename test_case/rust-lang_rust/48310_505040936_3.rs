
% ./build/x86_64-unknown-linux-gnu/llvm/bin/opt -lint -o /dev/null earlycsed-48310.ll
Undefined behavior: Call argument type mismatches callee parameter type
  invoke void bitcast (void (%Bar*)* @"_ZN59_$LT$narrowed_48310..Bar$u20$as$u20$narrowed_48310..Foo$GT$6borrow17hdd495dfcc8d99ffcE" to void ({}*)*)({}* align 1 %9)
          to label %bb3 unwind label %cleanup
