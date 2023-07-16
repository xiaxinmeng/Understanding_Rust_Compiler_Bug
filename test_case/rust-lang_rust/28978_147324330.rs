
#if defined(__USE_FILE_OFFSET64) || defined(__LP64__)
typedef int64_t off_t;
typedef off_t loff_t;
typedef loff_t off64_t;
#else
/* This historical accident means that we had a 32-bit off_t on 32-bit architectures. */
typedef __kernel_off_t off_t;
typedef __kernel_loff_t loff_t;
typedef loff_t off64_t;
#endif
