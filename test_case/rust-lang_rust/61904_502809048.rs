bash
#!/bin/sh
printf '#include <stdio.h>\nchar s[] = "' > hello.c
python -c 'print("h" * 4294967296)' >> hello.c
truncate -s -1 hello.c

printf '";' >> hello.c
cat >> hello.c << EOF

int main()
{
  printf("%s\n", s);
  return 0;
}
EOF
gcc hello.c -o hello_gcc
clang hello.c -o hello_clang # crash
