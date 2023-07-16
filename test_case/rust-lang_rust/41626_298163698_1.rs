
clang -c t.c
objdump -h t.o|grep -A1 debug_gdb_scripts
