 C
__declspec(dllexport) int foo = 1;
int* _imp__foo = &foo;

__declspec(dllexport) int bar() { return 2; }
void* _imp__bar = &bar;
