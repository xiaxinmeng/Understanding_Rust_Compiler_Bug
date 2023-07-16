bash
$ for var in $TMP $TEMP $USERPROFILE; do echo $(cygpath -w $var); done
C:\Users\basix\=
C:\Users\basix\AppData\Local\Temp
C:\Users\basix
