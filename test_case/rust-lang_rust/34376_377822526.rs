
$ cat /dev/null | false
$ echo ${PIPESTATUS[*]}
0 1
$ cat /dev/null | does-not-exist
bash: does-not-exist: command not found
$ echo ${PIPESTATUS[*]}
0 127
