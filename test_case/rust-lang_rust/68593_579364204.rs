
* Memory allocation functions malloc, calloc, realloc, reallocarray, valloc,
  pvalloc, memalign, and posix_memalign fail now with total object size
  larger than PTRDIFF_MAX.  This is to avoid potential undefined behavior with
  pointer subtraction within the allocated object, where results might
  overflow the ptrdiff_t type.
