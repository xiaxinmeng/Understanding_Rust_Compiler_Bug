
/* Use gcc_assert(EXPR) to test invariants.  */
#if ENABLE_ASSERT_CHECKING
#define gcc_assert(EXPR)                                                \
   ((void)(!(EXPR) ? fancy_abort (__FILE__, __LINE__, __FUNCTION__), 0 : 0))
#elif (GCC_VERSION >= 4005)
#define gcc_assert(EXPR)                                                \
  ((void)(__builtin_expect (!(EXPR), 0) ? __builtin_unreachable (), 0 : 0))
#else
/* Include EXPR, so that unused variable warnings do not occur.  */
#define gcc_assert(EXPR) ((void)(0 && (EXPR)))
#endif
