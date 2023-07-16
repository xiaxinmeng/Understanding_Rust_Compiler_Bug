 bash
/gcc/rust/util -I ../../gccrs/gcc/rust/metadata ../../gccrs/gcc/rust/backend/rust-constexpr.cc
In file included from ../../gccrs/gcc/rust/backend/rust-constexpr.cc:20:
../../gccrs/gcc/rust/backend/rust-constexpr.cc: In function ‘bool Rust::Compile::build_data_member_initialization(tree, vec<constructor_elt, va_gc>**)’:
../../gccrs/gcc/rust/backend/rust-tree.h:1324:55: error: invalid use of incomplete type ‘struct lang_type’
 1324 |   (CLASS_TYPE_P (NODE) && LANG_TYPE_CLASS_CHECK (NODE)->anon_aggr)
      |                                                       ^~
../../gccrs/gcc/rust/backend/rust-constexpr.cc:3833:16: note: in expansion of macro ‘ANON_AGGR_TYPE_P’
 3833 |       else if (ANON_AGGR_TYPE_P (TREE_TYPE (aggr)))
      |                ^~~~~~~~~~~~~~~~
In file included from ../../gccrs/gcc/tree.h:23,
                 from ../../gccrs/gcc/rust/backend/rust-constexpr.h:21,
                 from ../../gccrs/gcc/rust/backend/rust-constexpr.cc:17:
../../gccrs/gcc/tree-core.h:1740:10: note: forward declaration of ‘struct lang_type’
 1740 |   struct lang_type *lang_specific;
      |          ^~~~~~~~~
make[2]: *** [../../gccrs/gcc/rust/Make-lang.in:360: rust/rust-constexpr.o] Error 1
make[2]: Leaving directory '/home/mahad/Desktop/DrMahad/gccrs-build/gcc'
make[1]: *** [Makefile:4571: all-gcc] Error 2
make[1]: Leaving directory '/home/mahad/Desktop/DrMahad/gccrs-build'
make: *** [Makefile:1006: all] Error 2

