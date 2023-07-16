
$ gcc -S test.c -o - -g2 -g0 | grep '\.debug_info' # no output
$ gcc -S test.c -o - -g0 -g2 | grep '\.debug_info'
.section .debug_info
