plain
.................................................

failures:

---- [ui] tests/ui/lexer/tricky-exponents.rs stdout ----
normalized run.stdout:
1e_3_
1e____3
1e____3_3__
1e+____3
---
2.0e3em
0e_
1e_a
1e_a_
1e + +
1e + ___
1e - _ -
1e + ____a
1e - ____a_a__
3.3e_____
1.0e3foo
# aabbcc
# aabb11
# 112233
# 112233
# 1122aa
# 112e33
# 112e3a
# 11223e
# 1122ea
7ad85a2c - f2d0 - 11fd - afd0 - b3104db0cb68
7ad85a2c - f2d0 - 11ed - afd0 - b3104db0cb68
7ad85a2c - f2d0 - 111e - afd0 - b3104db0cb68


The actual run.stdout differed from the expected run.stdout.
The actual run.stdout differed from the expected run.stdout.
Actual run.stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lexer/tricky-exponents/tricky-exponents.run.stdout
error: 1 errors occurred comparing run output.
status: exit status: 0
status: exit status: 0
command: cd "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lexer/tricky-exponents" && RUST_TEST_THREADS="16" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lexer/tricky-exponents/a"
1e_3
1e_3_
1e____3
1e____3_3__
---
2.0e3em
0e_
1e_a
1e_a_
1e + +
1e + ___
1e - _ -
1e + ____a
1e - ____a_a__
3.3e_____
1.0e3foo
# aabbcc
# aabb11
# 112233
# 112233
# 1122aa
# 112e33
# 112e3a
# 11223e
# 1122ea
7ad85a2c - f2d0 - 11fd - afd0 - b3104db0cb68
7ad85a2c - f2d0 - 11ed - afd0 - b3104db0cb68
7ad85a2c - f2d0 - 111e - afd0 - b3104db0cb68
stderr: none



