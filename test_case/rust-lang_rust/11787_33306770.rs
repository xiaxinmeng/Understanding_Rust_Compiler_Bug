 sh
$ git grep 'use std::[^:]*;' | wc -l
936
$ git grep 'use std::[^:]*::' | wc -l
1096
$ git grep 'use std::[^:]*::[^A-Z]'  | wc -l
650
