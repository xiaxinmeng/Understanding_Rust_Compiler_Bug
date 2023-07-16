r2
    $ valgrind ./tmp_nightly
    [..]
    ==9866== HEAP SUMMARY:
    ==9866==     in use at exit: 32 bytes in 1 blocks
    ==9866==   total heap usage: 111 allocs, 110 frees, 9,275 bytes allocated
    ==9866== 
    ==9866== LEAK SUMMARY:
    ==9866==    definitely lost: 0 bytes in 0 blocks
    ==9866==    indirectly lost: 0 bytes in 0 blocks
    ==9866==      possibly lost: 0 bytes in 0 blocks
    ==9866==    still reachable: 32 bytes in 1 blocks
    ==9866==         suppressed: 0 bytes in 0 blocks
    ==9866== Rerun with --leak-check=full to see details of leaked memory
    [..]
    $ valgrind ./tmp_stable 
    [..]
    ==9879== HEAP SUMMARY:
    ==9879==     in use at exit: 32 bytes in 1 blocks
    ==9879==   total heap usage: 37 allocs, 36 frees, 5,552 bytes allocated
    ==9879== 
    ==9879== LEAK SUMMARY:
    ==9879==    definitely lost: 0 bytes in 0 blocks
    ==9879==    indirectly lost: 0 bytes in 0 blocks
    ==9879==      possibly lost: 0 bytes in 0 blocks
    ==9879==    still reachable: 32 bytes in 1 blocks
    ==9879==         suppressed: 0 bytes in 0 blocks
    ==9879== Rerun with --leak-check=full to see details of leaked memory
    [..]
    