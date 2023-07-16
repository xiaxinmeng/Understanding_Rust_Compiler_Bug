
$ ln -s contrib/clang-format .clang-format
...
$ git add some_files
$ git clang-format
<this adds style changes needed for staged changes as unstaged changes >
$ git add some_files
$ git commit .....
