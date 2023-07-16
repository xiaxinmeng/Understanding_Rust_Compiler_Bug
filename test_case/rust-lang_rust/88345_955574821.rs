
    rg -ce "$pattern" $include_dirs | string replace -r '^.*:' '' | paste -s -d+ - | bc
    