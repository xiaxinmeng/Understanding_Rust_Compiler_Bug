 bash
git branch
* master

git log | head -1
commit a8a847e30d206f5ff913c5286f92bce427f0f7f3

./x.py clean

./x.py test library/std

cd ./build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps

LD_LIBRARY_PATH=. valgrind --leak-check=full --show-leak-kinds=all --track-origins=yes ./std-0841f8a90f3e426f --exact time::tests::instant_math

==1140484== Memcheck, a memory error detector
==1140484== Copyright (C) 2002-2022, and GNU GPL'd, by Julian Seward et al.
==1140484== Using Valgrind-3.19.0 and LibVEX; rerun with -h for copyright info
==1140484== Command: ./std-0841f8a90f3e426f --exact time::tests::instant_math
==1140484== 

running 1 test
test time::tests::instant_math ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 951 filtered out; finished in 0.10s

==1140484== 
==1140484== HEAP SUMMARY:
==1140484==     in use at exit: 0 bytes in 0 blocks
==1140484==   total heap usage: 682 allocs, 682 frees, 168,443 bytes allocated
==1140484== 
==1140484== All heap blocks were freed -- no leaks are possible
==1140484== 
==1140484== For lists of detected and suppressed errors, rerun with: -s
==1140484== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)

