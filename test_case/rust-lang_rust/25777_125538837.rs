 C
/* NOTE: always allocate new memory for val! DON'T USE STATIC OR STACK MEMORY!!! */
#define SET_STR_RESULT(res, val)        \
(                       \
    (res)->type |= AR_STRING,       \
    (res)->str = (char *)(val)      \
)
