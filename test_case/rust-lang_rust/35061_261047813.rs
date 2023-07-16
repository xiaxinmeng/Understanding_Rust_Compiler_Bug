
 #!/bin/bash 

filtered_args=""

for arg; do

    if [ "$arg" != "-pie" ]
    then
        filtered_args="$filtered_args $arg"

    fi

done

command="cc $filtered_args"

echo " $command" > /tmp/linker

eval "$command"

