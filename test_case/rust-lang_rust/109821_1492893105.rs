shell
cat >foo.c <<EOF
void foo() {}
EOF
cat >bar.c <<EOF
void foo();

void main() { foo(); }
EOF
gcc -c foo.c -o foo.o
ar q foo.a foo.o
gcc -c bar.c -o bar.o
ar q bar.a bar.o
gcc bar.a foo.a -o test # use comes before definition => works
gcc foo.a bar.a -o test # definition comes before use => doesn't work
/usr/bin/ld: bar.a(bar.o): in function `main':
bar.c:(.text+0xa): undefined reference to `foo'
collect2: error: ld returned 1 exit status
