
../../third_party/llvm-build/Release+Asserts/bin/clang++ -fuse-ld=lld obj/build/rust/std/libstd.rlib \
  obj/build/rust/std/libaddr2line.rlib obj/build/rust/std/libadler.rlib obj/build/rust/std/liballoc.rlib \
  obj/build/rust/std/libcfg_if.rlib obj/build/rust/std/libcompiler_builtins.rlib \
  obj/build/rust/std/libcore.rlib obj/build/rust/std/libgetopts.rlib obj/build/rust/std/libgimli.rlib \
  obj/build/rust/std/libhashbrown.rlib obj/build/rust/std/liblibc.rlib obj/build/rust/std/libmemchr.rlib \
  obj/build/rust/std/libminiz_oxide.rlib obj/build/rust/std/libobject.rlib obj/build/rust/std/libpanic_abort.rlib \
  obj/build/rust/std/libpanic_unwind.rlib obj/build/rust/std/libproc_macro.rlib \
  obj/build/rust/std/librustc_demangle.rlib obj/build/rust/std/libstd_detect.rlib obj/build/rust/std/libtest.rlib \
  obj/build/rust/std/libunicode_width.rlib obj/build/rust/std/libunwind.rlib \
  -o ./main \
  -Wl,--start-group ./main.o obj/build/rust/std/remap_alloc/remap_alloc.o ${L} ./libmalloc_wrapper.so -Wl,--end-group \
  -ldl -lpthread -lrt -lgmodule-2.0 -lgobject-2.0 -lgthread-2.0 -lglib-2.0 -luuid
