
cp ~/rust-intel.ll .

# Following allows to build llvm-ir from intel on sparc (e.g. hashes differs)
gsed -i 's/target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"/target datalayout = "E-m:e-i64:64-n32:64-S128"/' rust-intel.ll
gsed -i 's/target triple = "x86_64-pc-solaris"/target triple = "sparcv9-sun-solaris"/' rust-intel.ll
gsed -i 's/h25e88cdc4b742d37/868f8880dc2af3fb/' rust-intel.ll
gsed -i 's/h62ae80c5f7549c57/h695fc56fad203862/' rust-intel.ll 
gsed -i 's/hb1a77afe5cf34a1f/h12114874ef566064/' rust-intel.ll
gsed -i 's/h01dfd331a3ddfd23/hde26e71a5da81bd3/' rust-intel.ll
gsed -i 's/h92300f5a5dcbe9fc/h6e897821881fc078/' rust-intel.ll
gsed -i 's/h260bdf0389a41d93/he0edee3f5f00176d/' rust-intel.ll
gsed -i 's/h68259f12b9557c97/hb3a671a76eb88d18/' rust-intel.ll
gsed -i 's/h93039a1a453ca6fa/h89b991e782a34d3c/' rust-intel.ll
gsed -i 's/h1cedb2ea3dec2dd2/h565f072da92eba4a/' rust-intel.ll
gsed -i 's/ha4d058251336ce5a/hce1c61317f4621e6/' rust-intel.ll
gsed -i 's/hd203f6187fbbf08f/h23743788f08f09ac/' rust-intel.ll
gsed -i 's/h887a1a8a37a961fb/h92ddcfd6dccbb247/' rust-intel.ll
gsed -i 's/868f8880dc2af3fb/h868f8880dc2af3fb/' rust-intel.ll

# Without this I'm getting error [1]
gsed -i 's/, file: !6)/)/' rust-intel.ll

llc --filetype=obj -o test.o rust-intel.ll
gcc -c -o myfunc.o src/myfunc.c
gcc -o test test.o myfunc.o -L /usr/lib/rustlib/sparcv9-sun-solaris/lib/ -lstd-899f7b6f82885664
LD_LIBRARY_PATH=/usr/lib/rustlib/sparcv9-sun-solaris/lib/ ./test
