 c++
__attribute__((noinline))
void zzz() {}

__attribute__((always_inline))
void inlined_fn() { 
    {
    int a = 1;
    zzz(); // #break
    }
}

int main() {
    inlined_fn();
    return 0;
}
