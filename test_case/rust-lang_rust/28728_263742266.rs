C
void foo() { while (1) { } }

void create_null() {
        foo();

        int i = 0;
        while (i < 100) { i += 1; }
}

__attribute__((noreturn))
void use_null() {
        __builtin_unreachable();
}


int main() {
        create_null();
        use_null();
}
