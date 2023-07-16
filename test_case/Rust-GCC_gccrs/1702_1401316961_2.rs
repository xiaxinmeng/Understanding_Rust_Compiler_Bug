bash
/usr/bin/ld: rust/rust-lang.o: in function `gt_ggc_mx_lang_tree_node(void*)':
/home/mahad/Desktop/DrMahad/gccrs-build/gcc/./gt-rust-rust-lang.h:369: undefined reference to `gt_ggc_mx_lang_type(void*)'
/usr/bin/ld: /home/mahad/Desktop/DrMahad/gccrs-build/gcc/./gt-rust-rust-lang.h:344: undefined reference to `gt_ggc_mx_lang_type(void*)'
/usr/bin/ld: rust/rust-lang.o: in function `gt_pch_nx_lang_tree_node(void*)':
/home/mahad/Desktop/DrMahad/gccrs-build/gcc/./gt-rust-rust-lang.h:830: undefined reference to `gt_pch_nx_lang_type(void*)'
/usr/bin/ld: /home/mahad/Desktop/DrMahad/gccrs-build/gcc/./gt-rust-rust-lang.h:855: undefined reference to `gt_pch_nx_lang_type(void*)'
collect2: error: ld returned 1 exit status
make[2]: *** [../../gccrs/gcc/rust/Make-lang.in:175: rust1] Error 1
make[2]: Leaving directory '/home/mahad/Desktop/DrMahad/gccrs-build/gcc'
make[1]: *** [Makefile:4571: all-gcc] Error 2
make[1]: Leaving directory '/home/mahad/Desktop/DrMahad/gccrs-build'
make: *** [Makefile:1006: all] Error 2
