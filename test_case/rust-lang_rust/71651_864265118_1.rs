shell
#!/bin/bash

crate_name=0

for i in "$@" ; do
    if [[ $i == "--crate-name" ]] ; then
	crate_name=1
	continue
    elif [[ "$crate_name" == "1" ]] ; then
	if [[ "$i" == "crate-name-of-your-program" ]] ; then
	    set -- "$@" -C target-feature=+crt-static
	    break
	fi
    fi
    crate_name=0
done

exec rustc "$@"
