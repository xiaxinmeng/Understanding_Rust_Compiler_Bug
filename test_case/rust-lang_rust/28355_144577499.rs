 python
  # We disable the LLVM simplify-libcalls pass by default, since we
  # statically link in libc, and pnacl-opt is typically used for post-link
  # optimizations.  Changing one library call to another can lead
  # to undefined symbol errors since which definitions from libc are linked
  # in is already decided.
