
In file included from hex.c:1:
In file included from ./cache.h:4:
In file included from ./git-compat-util.h:160:
/usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
__NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
^
/usr/include/sys/stat.h:434:8: note: previous definition is here
__NTH (stat (__const char *__path, struct stat *__statbuf))
^ 
