 c
struct _opaque_pthread_t {
        long __sig;
        struct __darwin_pthread_handler_rec  *__cleanup_stack;
        char __opaque[__PTHREAD_SIZE__];
};
typedef struct _opaque_pthread_t *__darwin_pthread_t;
