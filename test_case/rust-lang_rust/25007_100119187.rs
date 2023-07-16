 sh
# Use `md5sum` on GNU platforms, and `md5 -q` on BSD
system_md5sum=$(which md5sum)
system_md5=$(which md5)
if [ -n "$system_md5sum" ]
then
    CFG_HASH_COMMAND="$system_md5sum | head -c 8"
elif [ -n "$system_md5" ]
then
    CFG_HASH_COMMAND="$system_md5 -q | head -c 8"
else
    err 'either `md5sum` or `md5` must be installed'
fi
putvar CFG_HASH_COMMAND
