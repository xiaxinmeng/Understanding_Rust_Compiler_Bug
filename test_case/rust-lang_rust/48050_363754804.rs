sh
check_sums() {
    local _sumfile="$1"

    ...

    # run rm -R "$_workdir" || return 1     # <-- line 1447. comment out this line

    return $_sum_retval
}

...

download_and_check() {

    ...

    if [ $? != 0 ]; then
        # Whatever's in the cache doesn't add up. Delete it.
        # ignore rm -R "$_cache_dir"       # <-- line 1708. comment out this line
        return 1
    fi

}
