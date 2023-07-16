sh
#!/bin/sh
// stdin_empty.sh

use_real_rustc=0

for flag in "$@"; do
	case $flag in
		-v*|--print=file-names)
			use_real_rustc=1;
			break
			;;
		*) ;;
	esac
done

if [ $use_real_rustc = 1 ]; then
	exec rustc +$RUSTUP_TOOLCHAIN "$@"
fi

timeout .1 bash -c 'read -r x'
status=$?
if [ $status = 124 ]; then
	# stdin not closed (we timed out)
	exit 0
else
	# stdin closed
	exit 1
fi
