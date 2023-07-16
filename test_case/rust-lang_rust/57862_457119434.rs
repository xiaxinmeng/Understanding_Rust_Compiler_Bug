
In file included from zconf.tab.c:212:0:
zconf.hash.c:167:1: error: conflicting types for 'kconf_id_lookup'
 kconf_id_lookup (register const char *str, register size_t len)
 ^~~~~~~~~~~~~~~
zconf.hash.c:34:31: note: previous declaration of 'kconf_id_lookup' was here
 static const struct kconf_id *kconf_id_lookup(register const char *str, register unsigned int len);
                               ^~~~~~~~~~~~~~~
make[2]: *** [zconf.tab.o] Error 1
make[2]: *** Waiting for unfinished jobs....
make[1]: *** [build-lib-kconfig] Error 2
make: *** [build] Error 2
