sh
find . -name '*.txt' -print0 | 
    while IFS= read -r -d $'\0' line; do 
        # do something with "$line"
    done
