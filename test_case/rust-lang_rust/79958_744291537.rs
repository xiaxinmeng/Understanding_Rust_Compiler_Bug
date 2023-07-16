shell
$ ./x.py test --rustc-args="-Zinstrument-coverage" src/test/ui/associated-types/associated-types-ICE-when-projecting-out-of-err.rs
$ ./x.py test --rustc-args="-Zinstrument-coverage -Zsymbol-mangling-version=legacy" src/test/ui/const-generics/issues/issue-62579-no-match.rs 
