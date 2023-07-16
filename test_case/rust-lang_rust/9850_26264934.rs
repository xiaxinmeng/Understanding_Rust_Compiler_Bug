 c
int main() {
    short port = 0;
    int val = 0;
    asm("in %1, %0" : "a"(val) : "d"(port));
}
