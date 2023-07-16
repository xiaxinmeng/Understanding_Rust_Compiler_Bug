shell
> rg -A6 'unused borrow that must be used' | rg '&(mut)?\s*\w+\[' | wc -l
1020
