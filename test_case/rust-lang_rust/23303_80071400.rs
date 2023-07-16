
mkdir subdir
echo "void dummy(void){}" > subdir/dummy.c
gcc -shared subdir/dummy.c -o subdir/libdummy.so
echo  >main.rs '#[link(name = "dummy", kind = "dylib")]'
echo >>main.rs 'extern { fn dummy(); }'
echo >>main.rs 'fn main() { unsafe { dummy(); } }'
rustc main.rs -L "" -Lsubdir
