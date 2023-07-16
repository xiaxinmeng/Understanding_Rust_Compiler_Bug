
run doc-trpl-trait-objects [x86_64-unknown-linux-gnu]
test The_Stack_0 ... [32mok(B[m

running 11 tests
test _7 ... [33mignored(B[m
test _8 ... [33mignored(B[m
test _9 ... [33mignored(B[m
test _10 ... [31mFAILED(B[m
test The_Stack_2 ... [32mok(B[m
test Arguments_and_borrowing_0 ... [32mok(B[m
test The_Stack_1 ... [32mok(B[m
test A_complex_example_0 ... [32mok(B[m
test The_Heap_0 ... [32mok(B[m


...


failures:

---- _10 stdout ----
    <anon>:2:59: 2:60 error: unknown start of token: `
<anon>:2     error: cannot convert to a trait object because trait `core::clone::Clone` is not object-safe [E0038]
                                                                   ^
thread '_10' panicked at 'Box<Any>', src/libsyntax/parse/lexer/mod.rs:167



failures:
    _10
