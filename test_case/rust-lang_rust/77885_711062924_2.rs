c
int size;

void foo(void*);

int main() {
    foo(alloca(size));
}
