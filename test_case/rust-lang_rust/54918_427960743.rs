
error[E0668]: malformed inline assembly============================>   ] 26/27: std                                                                                                                         
  --> ~/GitHub/rust/src/libcore/../stdsimd/coresimd/x86/cpuid.rs:64:9
   |
64 | /         asm!("cpuid\n"
65 | |              : "={eax}"(r.eax), "={ebx}"(r.ebx), "={ecx}"(r.ecx), "={edx}"(r.edx)
66 | |              : "{eax}"(leaf), "{ecx}"(sub_leaf)
67 | |              : "rbx" :);
   | |________________________^

error[E0668]: malformed inline assembly
  --> ~/GitHub/rust/src/libcore/../stdsimd/coresimd/x86/xsave.rs:91:5
   |
91 |     asm!("xgetbv" : "={eax}"(eax), "={edx}"(edx) : "{ecx}"(xcr_no));
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to 2 previous errors
