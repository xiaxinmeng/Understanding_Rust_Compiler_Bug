 bash
probe CFG_VALGRIND         valgrind
if [ -z "$CFG_VALGRIND" ]
then
    if [ ! -z "$CFG_ENABLE_VALGRIND" ] || [ -z "$CFG_DISABLE_VALGRIND_RPASS" ]
    then
      err "Valgrind not found, but wanted. You may want to add --disable-valgrind-rpass"                      
    fi
fi
