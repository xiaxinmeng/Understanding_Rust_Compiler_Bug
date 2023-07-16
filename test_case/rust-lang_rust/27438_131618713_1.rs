 C
__declspec(dllimport) int foo;
__declspec(dllimport) int bar();

int main()
{
    int f = foo;
    int b = bar();
    printf("%d %d\n", f, b);
    return 0;
}
