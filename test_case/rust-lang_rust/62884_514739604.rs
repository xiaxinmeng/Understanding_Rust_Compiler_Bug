
$ echo " " >> src/librustc/lib.rs
$ ./x.py build -i --stage 1 --keep-stage 1 src/librustc
[...]
Build completed successfully in 0:02:31

$ echo " " >> src/librustc_codegen_ssa/lib.rs
$ ./x.py build -i --stage 1 --keep-stage 1 src/librustc_codegen_ssa/lib.rs
[...]
Build completed successfully in 0:00:51

$ echo " " >> src/librustc_driver/lib.rs
$ ./x.py build -i --stage 1 --keep-stage 1 src/librustc_driver
[...]
Build completed successfully in 0:00:39 # Almost there
