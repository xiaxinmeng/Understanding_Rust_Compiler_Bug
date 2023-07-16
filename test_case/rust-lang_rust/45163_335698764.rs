
[00:02:12] /checkout/src/liballoc_jemalloc/../jemalloc/src/pages.c: In function 'os_overcommits_proc':
[00:02:12] /checkout/src/liballoc_jemalloc/../jemalloc/src/pages.c:357:6: error: 'O_CLOEXEC' undeclared (first use in this function)
[00:02:12]       O_CLOEXEC);
[00:02:12]       ^
...
[00:02:12] /checkout/src/liballoc_jemalloc/../jemalloc/src/prof.c: In function 'prof_open_maps':
[00:02:12] /checkout/src/liballoc_jemalloc/../jemalloc/src/prof.c:1412:34: error: 'O_CLOEXEC' undeclared (first use in this function)
[00:02:12]   mfd = open(filename, O_RDONLY | O_CLOEXEC);
[00:02:12]                                   ^
