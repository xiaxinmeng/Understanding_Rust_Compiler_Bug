bash
#!/bin/bash

(
        date
        pargs $$
        echo
        unset LD_ALTEXEC
        /usr/bin/ld -Dfiles,unused "$@"
        rc=$?
        echo
        echo exit: $rc
        echo
) >>/var/tmp/capld.args.txt 2>&1

exit $rc
