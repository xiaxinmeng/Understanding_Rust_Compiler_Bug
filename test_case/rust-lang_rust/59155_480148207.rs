
/usr/bin/ld: /home/thayne/devel/rust-lang/build/x86_64-unknown-linux-gnu/test/run-pass/intrinsics/intrinsic-atomics/a.intrinsic_atomics.7rcbfp3g-cgu.2.rcgu.o: in function `intrinsic_atomics::main':
intrinsic_atomics.7rcbfp3g-cgu.2:(.text._ZN17intrinsic_atomics4main17h1af59ebac19920e7E+0x346): undefined reference to `__llvm_memcpy_element_unordered_atomic_4'
/usr/bin/ld: intrinsic_atomics.7rcbfp3g-cgu.2:(.text._ZN17intrinsic_atomics4main17h1af59ebac19920e7E+0x37d): undefined reference to `__llvm_memmove_element_unordered_atomic_4'
/usr/bin/ld: intrinsic_atomics.7rcbfp3g-cgu.2:(.text._ZN17intrinsic_atomics4main17h1af59ebac19920e7E+0x3b2): undefined reference to `__llvm_memset_element_unordered_atomic_4'
collect2: error: ld returned 1 exit status
