 c
// this non-short-circuiting example behaves just like the unoptimized rust one,
// with my manual bool optimization applied
unsigned long foo(unsigned long i) {return (i % 5 == 1 & i % 4 == 0) ? i % 4 : 0;}
