
$ echo __GNUC__ __GNUC_MINOR__ __GNUC_PATCHLEVEL__ | gcc -x c -E - | grep -ve '^#'
9 3 0
$ echo __clang_major__ __clang_minor__ __clang_patchlevel__ | clang -x c -E - | grep -ve '^#'
13 0 1
