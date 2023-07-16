 C
#include <windows.h>
int main() {
    asm("movl $1234, %fs:0x14");
    MessageBoxA(0, "abc", "abc", 0);
}
