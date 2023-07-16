compile_fail,E0277\n// here we declare the Foo trait with a bar method\ntrait Foo {\n    fn bar(&self);\n}\n\n// we now declare a function which takes an object implementing the Foo trait\nfn some_func<T: Foo>(foo: T) {\n    foo.bar();\n}\n\nfn main() {\n    // we now call the meth

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02037db2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
