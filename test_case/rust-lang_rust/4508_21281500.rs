
struct foo_struct { int a; foo_struct(int _a): a(_a) { } };
typedef foo_struct foo;
foo f = foo(0);
