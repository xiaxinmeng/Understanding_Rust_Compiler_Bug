bash
#!/bin/bash

args=()

for arg in "$@"; do
	if [[ $arg = *"Bdynamic"* ]]; then
		args+=() # we do not want this arg
	elif [[ $arg = *"crti.o"* ]]; then
		args+=("$arg" "/usr/arm-unknown-linux-musleabi/lib/gcc/arm-unknown-linux-musleabi/6.3.0/crtbeginT.o" "-Bstatic")
	elif [[ $arg = *"crtn.o"* ]]; then
		args+=("-lgcc" "-lgcc_eh" "-lc" "/usr/arm-unknown-linux-musleabi/lib/gcc/arm-unknown-linux-musleabi/6.3.0/crtend.o" "$arg")
	else
		args+=("$arg")
	fi
done
#echo "RUNNING WITH ARGS: ${args[@]}"
arm-unknown-linux-musleabi-real-g++ "${args[@]}"
