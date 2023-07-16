 c
// optimized in clang to ret 0, but I think it does short-circuiting differently,
// so it's equivalent to the optimized rustc one
unsigned long foo(unsigned long i) {return (i % 5 == 1 && i % 4 == 0) ? i % 4 : 0;}
