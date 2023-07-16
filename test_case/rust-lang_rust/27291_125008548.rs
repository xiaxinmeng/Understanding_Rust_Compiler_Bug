
$ echo '#include <sys/types.h>' | gcc - -E -o - | grep mode_t
typedef unsigned int __mode_t;
typedef __mode_t mode_t;
