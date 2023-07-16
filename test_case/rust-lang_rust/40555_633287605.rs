
$ cat clang_asm.c 
int main() {
    asm("nowayisthisavalidinstruction");
    return 0;
}
$ clang -target arm-unknown-linux-gnueabihf clang_asm.c
<inline asm>:1:2: error: invalid instruction
        nowayisthisavalidinstruction
        ^
1 error generated.
$ clang -target arm-unknown-linux-gnueabihf clang_asm.c -O1
clang_asm.c:2:9: error: invalid instruction
    asm("nowayisthisavalidinstruction");
        ^
<inline asm>:1:2: note: instantiated into assembly here
        nowayisthisavalidinstruction
        ^~~~~~~~~~~~~~~~~~~~~~~~~~~~
1 error generated.
