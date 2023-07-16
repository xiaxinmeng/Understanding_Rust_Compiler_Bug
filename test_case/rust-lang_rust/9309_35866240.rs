 sh
$ git grep 'extern *\("C"\)\? *{' | wc -l # extern { ... }, or extern "C" { ... }
221
$ git grep 'extern *".*" *{' | grep -v "C" | wc -l # extern "non-C abi" { ... }
34
