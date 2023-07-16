c
extern int bar(int, char *);

char *s = "Foo";

int foo(int n)
{
    return bar(n, s);
}
