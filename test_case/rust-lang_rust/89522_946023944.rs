
 16 #define WRAPPER_PRE_CHECKS() \
 17 ({ \
 18         /* pthread_atfork(sb_lock, sb_unlock, sb_unlock); */ \
 19         sb_lock(); \
 20         result = SB_HIDDEN_FUNC(WRAPPER_NAME)(WRAPPER_ARGS_FULL); \
 21         sb_unlock(); \
 22         false; \
 23 })
