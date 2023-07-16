c
extern void f(char*);

static char *s = "blah";

void test(void) {
    f(s);
}
