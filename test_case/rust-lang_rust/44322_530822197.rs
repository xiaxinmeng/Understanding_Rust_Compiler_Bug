
gcc main.c -Llib1/target/release/ -Llib2/target/release/ -llib1 -llib2 -lpthread -ldl
lib2/target/release//liblib2.a(lib2-929bb870274e5eac.lib2.8t8q9ckg-cgu.0.rcgu.o): In function `rust_eh_personality':
/rustc/eae3437dfe991621e8afdc82734f4a172d7ddf9b//src/libpanic_abort/lib.rs:106: multiple definition of `rust_eh_personality'
lib1/target/release//liblib1.a(lib1-d10e44ca50aa3d7b.lib1.eqtft63h-cgu.0.rcgu.o):/rustc/eae3437dfe991621e8afdc82734f4a172d7ddf9b//src/libpanic_abort/lib.rs:106: first defined here
collect2: error: ld returned 1 exit status
Makefile:2: recipe for target 'a.out' failed
make: *** [a.out] Error 1
