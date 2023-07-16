' src/doc/trpl/ffi.md | paste -s -d ':\n' - | grep 'rust\|no_run')
do
    from=$(echo $block | cut -d: -f1);
    to=$(echo $block | cut -d: -f3);
    f=$(mktemp /tmp/doctest.XXX);
    head -$((to - 1)) src/doc/trpl/ffi.md | tail -$(($to - $from - 1)) | sed 's/^# //' >$f;
    mr ru nightly rustc $f >/dev/null 2>&1 && rm doctest;
    echo $from $?;
    rm $f;
done
