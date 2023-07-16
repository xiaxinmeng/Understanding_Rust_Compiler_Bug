
#!/bin/bash

declare -a FILTERARGS
for ITEM in "$@"
do
    case "$ITEM" in
    -Wl,-znow|-Wl,-zrelro|-Wl,--as-needed)
        # do nothing
        ;;
    *)
       FILTERARGS+=("$ITEM")
        ;;
    esac
done

echo "invoking: cc ${FILTERARGS[@]}"
cc ${FILTERARGS[@]}
