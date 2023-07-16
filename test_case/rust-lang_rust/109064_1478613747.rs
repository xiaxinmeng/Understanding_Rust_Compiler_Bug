
#if defined(__GLIBC__)
  // Current code for aarch64-unknown-linux-gnu
  unsigned long hwcap = getauxval(AT_HWCAP);
  _Bool result = (hwcap & HWCAP_ATOMICS) != 0;
#elif defined(MUSL)
  // Code for aarch64-unknown-linux-musl to detect LSE instructions, or just default +outline-atomics not supported.
  result = false;
#else
  #error Unsupported libc
#endif
