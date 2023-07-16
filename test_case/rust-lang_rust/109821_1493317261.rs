
cat >foo.c <<EOF
void foo() {}
EOF
cat >bar.c <<EOF
void foo();

void main() { foo(); }
EOF
gcc -c foo.c -o foo.o
gcc -c bar.c -o bar.o
ar q bar.a bar.o
gcc foo.o bar.a -o test
