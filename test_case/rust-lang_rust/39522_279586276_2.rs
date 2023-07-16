
mv src/etc/debugger_pretty_printers_common.py{,.backup} &&
    touch src/etc/debugger_pretty_printers_common.py &&
    ./x.py test src/test/debuginfo
