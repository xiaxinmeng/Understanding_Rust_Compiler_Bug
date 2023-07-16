
# echo 'fn main(){}' | rustc - -o true
# nm ./true | grep atexit
00034820 t atexit
         U __cxa_atexit@@GLIBC_2.1.3
         w __cxa_thread_atexit_impl@@GLIBC_2.18
00016ac0 t stats_print_atexit
# ./true
# echo $?
0
