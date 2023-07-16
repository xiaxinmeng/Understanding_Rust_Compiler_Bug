
./gcc/rust1 -flto -frust-dump-parse -Warray-bounds -fdump-tree-gimple ../gcc/testsuite/rust/compile/bad_file_name.txt.rs -o t.s              04:43:42
rust1: error: invalid character ‘.’ in crate name: ‘bad_file_name.txt’
../gcc/testsuite/rust/compile/bad_file_name.txt.rs: note: crate name inferred from this file
